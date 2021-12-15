#![allow(where_clauses_object_safety)]

use std::{
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    time::timeout,
};

use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use parking_lot::{Mutex, RwLock};
use trust_dns_resolver::{config::*, TokioAsyncResolver};

use packet::{varint, ProtoDec, ProtoEnc, RawPacket, Varint};

use crate::{
    logging::LogQueue,
    types::{DataQueue, Queues},
};

pub(crate) use mcore::types::Direction;
pub(crate) use plugin::EventHandler;
pub(crate) use packet::SharedState;

mod logging;
mod types;
mod utils;

const SHUTDOWN_CHECK_TIMEOUT: u64 = 100;

// This function puts all received packets (in chunks of 4096 bytes) in the receiving queue.
async fn receiver(
    mut rx: OwnedReadHalf,
    queue: Arc<DataQueue>,
    socket_name: &str,
    is_closed: Arc<AtomicBool>,
) {
    // This buffer is continually reused
    let mut buf = [0; 4096];
    loop {
        // The timeout is there to close the socket and thread if the connection is closed
        let n = match timeout(
            Duration::from_millis(SHUTDOWN_CHECK_TIMEOUT),
            rx.read(&mut buf),
        )
        .await
        {
            Ok(v) => match v {
                Ok(n) if n == 0 => {
                    log::warn!("Socket closed: {}", socket_name);
                    is_closed.store(true, Ordering::Release);
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    log::error!("Failed to read from socket: {}", e);
                    return;
                }
            },
            Err(_) => {
                if is_closed.load(Ordering::Relaxed) {
                    log::debug!("Stopping thread {} because socket was closed", socket_name);
                    return;
                } else {
                    continue;
                }
            }
        };
        queue.push(buf[0..n].to_vec());
    }
}

// This sends the data in the respective queues to the tx.
async fn sender(mut tx: OwnedWriteHalf, queue: Arc<DataQueue>, is_closed: Arc<AtomicBool>) {
    loop {
        if let Err(e) = tx
            .write_all(
                &(match timeout(Duration::from_millis(SHUTDOWN_CHECK_TIMEOUT), queue.pop()).await {
                    Ok(b) => b,
                    Err(_) => {
                        if is_closed.load(Ordering::Relaxed) {
                            log::debug!("Stopping sender because socket was closed",);
                            return;
                        } else {
                            continue;
                        }
                    }
                }),
            )
            .await
        {
            log::error!("Failed to write to socket: {}", e);
            break;
        };
    }
}

async fn parser(
    queues: Queues,
    shared_status: Arc<RwLock<SharedState>>,
    ciphers: Arc<Mutex<cipher::Ciphers>>,
    direction: Direction,
    is_closed: Arc<AtomicBool>,
    plugins: Arc<Mutex<Vec<Box<dyn EventHandler + Send>>>>,
    log_queue: Arc<LogQueue>,
) -> Result<(), ()> {
    let mut unprocessed_data = RawPacket::new();
    // functions is a list of all the packets that can be parsed
    let functions = protocol::current_protocol::Functions::new();
    let config = config_loader::get_config();

    // If this loop ever breaks, the thread is closed.
    loop {
        // This is the only place this function can hang, so it has a timeout to fix that.
        let new_data = match timeout(
            Duration::from_millis(SHUTDOWN_CHECK_TIMEOUT),
            match direction {
                Direction::Serverbound => queues.client_proxy.pop(),
                Direction::Clientbound => queues.server_proxy.pop(),
            },
        )
        .await
        {
            Ok(new_data) => new_data,
            Err(_) => {
                if is_closed.load(Ordering::Relaxed) {
                    break;
                } else {
                    continue;
                }
            }
        };

        // Data from the server (clientbound) needs to be decrypted, that is done here.
        unprocessed_data.push_vec({
            if direction == Direction::Clientbound {
                ciphers.lock().sp_cipher.decrypt(new_data)
            } else {
                new_data
            }
        });

        // Sometimes multiple packets will be sent at once, so this keeps running until all the packets are dealth with.
        while !unprocessed_data.is_empty() {
            // A backup of the packet is made, so if the packet is incomplete it will be reset to that.
            let original_data = unprocessed_data.get_vec();

            let packet_length = match unprocessed_data.decode::<Varint>() {
                Ok(packet_length) => packet_length.into(),
                Err(_) => {
                    unprocessed_data.set(original_data);
                    break;
                }
            };

            // If there is not enough data in the unprocessed_data variable, it will restore the backup and wait for more data.
            if unprocessed_data.len() < packet_length {
                unprocessed_data.set(original_data);
                break;
            }

            // This packet will always have the exact amount of data to contain the packet, no more no less.
            let mut packet = packet::RawPacket::from(unprocessed_data.read(packet_length).unwrap());

            // A copy of the packet is made, this is to not have to recreate it if it doesn't get parsed or edited.
            let mut original_packet = RawPacket::new();
            original_packet.encode(&varint!(packet_length))?;
            original_packet.push_vec(packet.get_vec());

            // Uncompress if needed
            if shared_status.read().compress > 0 {
                let data_length: Varint = packet.decode().unwrap();
                if data_length > 0 {
                    let decompressed_packet = match decompress_to_vec_zlib(&packet.get_vec()) {
                        Ok(decompressed_packet) => decompressed_packet,
                        Err(why) => {
                            log::error!("Decompress error: {:?} {}", why, direction);
                            break;
                        }
                    };
                    packet.set(decompressed_packet);
                }
            }

            let packet_id = packet.decode::<Varint>()?;

            // Get the Fid of the current packet, if it doesn't get parsed set it to Unparsable
            let func_id =
                match functions.get_name(&direction, &shared_status.read().state, &packet_id) {
                    Some(func_name) => func_name,
                    None => &protocol::Fid::Unparsable,
                };

            let mut out_data = original_packet.get_vec();
            let mut to_direction = direction;

            if func_id == &protocol::Fid::Unparsable {
                // Encrypt the data if it wont get parsed. Othwerise, ecryption is done later.
                if direction == Direction::Serverbound {
                    out_data = ciphers.lock().ps_cipher.encrypt(out_data)
                }
            } else {
                // This arm runs if the data will get parsed.
                // This gets the actual functions to parse the packet
                let mut parsed_packet = match functions.get(func_id) {
                    Some(func) => func,
                    None => unreachable!(),
                };

                // The success variable is used becase some code needs to be executed regardless of if the packet parsed correct
                // otherwise the connection would fail as soon as one packet doesn't get parsed correctly.
                let success = if parsed_packet.decode(&mut packet).is_ok() {
                    if config.logging_packets.contains(&func_id.to_string())
                        || config.logging_packets.contains(&"*".to_string())
                    {
                        // The 3:4$ makes sure there is a consistant amount of spaces between the ] and the start of the packet info
                        log::info!(
                            "{} [{}]{3:4$} {}",
                            direction.to_string().yellow(),
                            func_id.to_string().blue(),
                            parsed_packet,
                            "",
                            config.print_buffer - func_id.to_string().len()
                        );
                    }
                    // This is for the JSON logging
                    log_queue.push(parsed_packet.clone());
                    true
                } else {
                    log::error!(
                        "Could not parse packet! {} {} {}",
                        packet_id,
                        func_id.to_string(),
                        direction
                    );
                    false
                };

                if success {
                    parsed_packet
                        .update_status(&mut shared_status.write())
                        .unwrap();

                    // Packet editing takes a lot more time, so it only gets executed if it is needed.
                    if parsed_packet.packet_editing() {
                        let mut shared_status_copy = shared_status.read().clone();
                        let mut shared_plugins = plugins.lock().clone();
                        match parsed_packet
                            .edit_packet(&mut shared_status_copy, &mut shared_plugins, &config)
                            .await
                        {
                            Ok(packet_vec) => {
                                if packet_vec.len() > 1 {
                                    // When multiple packet get sent back
                                    for (packet, new_direction) in packet_vec {
                                        let out_d = if shared_status_copy.compress == 0 {
                                            packet.get_data_uncompressed().unwrap()
                                        } else {
                                            packet
                                                .get_data_compressed(
                                                    shared_status_copy.compress as i32,
                                                )
                                                .unwrap()
                                        };
                                        match new_direction {
                                            Direction::Serverbound => {
                                                let out_d = ciphers.lock().ps_cipher.encrypt(out_d);
                                                queues.proxy_server.push(out_d);
                                            }
                                            Direction::Clientbound => {
                                                queues.proxy_client.push(out_d)
                                            }
                                        }
                                    }
                                    // Make sure the original data doesn't get sent anymore
                                    out_data.clear();
                                } else if packet_vec.is_empty() {
                                    // Send the original packet.
                                } else {
                                    // One packet
                                    let (packet, new_direction) = &packet_vec[0];
                                    to_direction = new_direction.to_owned();
                                    out_data = if shared_status_copy.compress == 0 {
                                        packet.get_data_uncompressed().unwrap()
                                    } else {
                                        packet
                                            .get_data_compressed(shared_status_copy.compress as i32)
                                            .unwrap()
                                    };
                                }
                                shared_status.write().set(shared_status_copy);
                                let mut locked_plugins = plugins.lock();
                                // This should probably be optimized?
                                locked_plugins.clear();
                                locked_plugins.append(&mut shared_plugins);
                            }
                            Err(_) => {
                                panic!("This should never happen");
                            }
                        };
                    }
                }

                if to_direction == Direction::Serverbound {
                    out_data = ciphers.lock().ps_cipher.encrypt(out_data)
                }

                if success
                    && parsed_packet
                        .post_send_update(&mut ciphers.lock(), &shared_status.read())
                        .is_err()
                {
                    panic!("Post send update failed, panicing.")
                }
            };

            match to_direction {
                Direction::Serverbound => queues.proxy_server.push(out_data),
                Direction::Clientbound => queues.proxy_client.push(out_data),
            }
        }
    }
    Ok(())
}

async fn handle_connection(
    mut client_stream: TcpStream,
    user_ip: String,
    connection_id: String,
) -> Result<(), ()> {
    let config = config_loader::get_config();

    // This shared state stores all *mutable* data that is needed in more than one thread.
    let shared_ciphers: Arc<Mutex<cipher::Ciphers>> = Arc::new(Mutex::new(cipher::Ciphers::new()));

    // This part reads data from the client (the first packet) into a buffer
    let mut buffer = Vec::new();
    client_stream.read_buf(&mut buffer).await.unwrap();

    // It tries to parse the first packet
    let mut initial_data = RawPacket::from(buffer);
    let packet_length: usize = initial_data.decode::<Varint>()?.into();
    let mut raw_first_packet = RawPacket::from(match initial_data.read(packet_length as usize) {
        Ok(p) => p,
        Err(_) => return Ok(()),
    });
    let packet_id: Varint = raw_first_packet.decode()?;

    // If the packet ID is not 0, it is not a valid minecraft packet.
    if packet_id != 0 {
        log::error!("Packet ID did not match handshaking packet, terminating connection...");
        // It returns OK because the connection was dealth with successfully, not because everything went like it should have.
        return Ok(());
    };

    // It then continues to parse the packet like it is a handshaking packet.
    let handshaking_packet =
        protocol::serverbound::handshaking::Handshake::decode_ret(&mut raw_first_packet);
    if handshaking_packet.is_err() {
        log::error!("Invalid handshake packet! Closing connection...");
        // Again, returning OK because everything was dealt with, no loose ends.
        return Ok(());
    };

    let handshaking_packet = handshaking_packet.unwrap();

    // It then gets the IP address of the actual server to connect to, minus the domain suffix.
    let ip = match handshaking_packet
        .server_address
        .strip_suffix(&config.domain_suffix)
    {
        Some(m) => m,
        None => {
            log::error!(
                "Could not strip suffix of {}",
                handshaking_packet.server_address
            );
            return Ok(());
        }
    }
    .to_string();

    // It looks if there is an SRV record present on the domain, if there is it uses that.
    log::debug!("Resolving SRV recrod for ip: {}", ip);
    let resolver =
        TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let lookup = resolver.srv_lookup(format!("_minecraft._tcp.{}", ip)).await;

    let address = match lookup {
        Ok(response) => {
            let record = response.iter().next().unwrap();

            let ip = record.target().to_string().trim_matches('.').to_string();
            log::debug!("IP after SRV resolution: {}", ip);
            ip
        }
        Err(_) => {
            // Othwerise it just uses the initial IP
            log::debug!("No different IP found after SRV resolution: {}", ip);
            ip.to_owned()
        }
    };

    // It converts the updated data back to a packet.
    let mut new_packet = protocol::serverbound::handshaking::Handshake {
        protocol_version: handshaking_packet.protocol_version,
        server_address: address.clone(),
        server_port: 25565,
        next_state: handshaking_packet.next_state,
    }
    .encode_packet()?
    // let mut new_packet = packet::Packet::from(
    // new_raw_packet,
    // functions::fid_to_pid(crate::functions::Fid::Handshake),
    // )
    .get_data_uncompressed()?;

    // It adds the remaining data that was sent in the first packet, to make sure no data gets lost.
    new_packet.append(&mut initial_data.get_vec());

    // It connects to the server, for now the port 25565 is hardcoded.
    log::info!("Connecting to IP {}", &address);
    let server_stream = match TcpStream::connect(&format!("{}:{}", &address, 25565)).await {
        Ok(stream) => stream,
        Err(err) => {
            log::error!("Could not connect to ip: {}", err);
            // As always, returning OK because nothing unhandled happend.
            return Ok(());
        }
    };
    log::info!("Connected...");

    // It then splits both TCP streams up in rx and tx
    let (crx, ctx) = client_stream.into_split();
    let (srx, stx) = server_stream.into_split();

    // The queues (except for logging) are in this struct, this is to keep the arguments organized.
    let queues = Queues {
        client_proxy: Arc::new(DataQueue::new()),
        proxy_client: Arc::new(DataQueue::new()),
        server_proxy: Arc::new(DataQueue::new()),
        proxy_server: Arc::new(DataQueue::new()),
    };

    // The data that might have been left over from the first packet is added to the queue.
    // This is done here because there is no need to create the queues when the server might never connect.
    queues.client_proxy.push(new_packet);

    // It creates a shared status where all data that is mutable or request specific is kept.
    let shared_status: Arc<RwLock<SharedState>> = Arc::new(RwLock::new(SharedState {
        // These values might not get used.
        access_token: config.player_auth_token,
        uuid: config.player_uuid,
        server_ip: address,
        connection_id,
        user_ip,
        ..SharedState::new()
    }));

    // These variables are set here, this is after something could have gone wrong,
    //    so they don't get created if they don't need to.
    let log_queue = Arc::new(LogQueue::new());
    let is_closed = Arc::new(AtomicBool::new(false));
    let plugins = Arc::new(Mutex::new(plugin_loader::get_plugins()));

    // Start a thread for logging the packets
    tokio::spawn({
        let log_path = format!("./logs/{}.txt", &shared_status.read().connection_id);
        let log_queue = log_queue.clone();
        async move { logging::logger(&log_path, log_queue).await }
    });

    // It then starts two threads to put all the received data from the RX channels into the queues
    tokio::spawn({
        let client_proxy_queue = queues.client_proxy.clone();
        let is_closed = is_closed.clone();
        async move { receiver(crx, client_proxy_queue, "client", is_closed).await }
    });
    tokio::spawn({
        let server_proxy_queue = queues.server_proxy.clone();
        let is_closed = is_closed.clone();
        async move { receiver(srx, server_proxy_queue, "server", is_closed).await }
    });

    // And it also starts two to put the queued data into the TX channels
    tokio::spawn({
        let proxy_client_queue = queues.proxy_client.clone();
        let is_closed = is_closed.clone();
        async move { sender(ctx, proxy_client_queue, is_closed).await }
    });
    tokio::spawn({
        let proxy_server_queue = queues.proxy_server.clone();
        let is_closed = is_closed.clone();
        async move { sender(stx, proxy_server_queue, is_closed).await }
    });

    // It then starts two parsers, one for each of the directions.
    // These parsers make sure the data is sent both ways and possibly edited and/or logged.
    tokio::spawn({
        let shared_status = shared_status.clone();
        let shared_ciphers = shared_ciphers.clone();
        let queues = queues.clone();
        let is_closed = is_closed.clone();
        let plugins = plugins.clone();
        let log_queue = log_queue.clone();
        async move {
            parser(
                queues,
                shared_status,
                shared_ciphers,
                Direction::Serverbound,
                is_closed,
                plugins,
                log_queue,
            )
            .await
        }
    });
    tokio::spawn({
        async move {
            parser(
                queues,
                shared_status,
                shared_ciphers,
                Direction::Clientbound,
                is_closed,
                plugins,
                log_queue,
            )
            .await
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load the logger, it has a fancy format with colours and it's spaced.
    Builder::from_default_env()
        .format(|buf, record| {
            let formatted_level = buf.default_styled_level(record.level());
            writeln!(buf, "{:<5} {}", formatted_level, record.args())
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    // Try to load config to make sure it works
    let config = config_loader::get_config();

    log::info!("Starting listener...");
    // Start listening on the ip waiting for new connections
    let mc_client_listener = match TcpListener::bind(config.listen_address).await {
        Ok(listener) => listener,
        Err(err) => panic!("Could not connect to server: {}", err),
    };

    loop {
        // If this continues, a new client is connected.
        let next_connection_id = utils::generate_connection_id();
        let (socket, socket_addr) = mc_client_listener.accept().await?;
        let ip = socket_addr.ip().to_string();
        log::info!(
            "Client connected, connection ID: {} IP: {}",
            next_connection_id,
            ip
        );
        // Start the client-handling thread (this will complete quickly)
        tokio::spawn(async move {
            handle_connection(socket, ip, next_connection_id)
                .await
                .unwrap();
        });
    }
}
