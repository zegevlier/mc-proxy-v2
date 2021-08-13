#![allow(where_clauses_object_safety)]

use std::{
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use parking_lot::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    time::timeout,
};
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

pub use plugin::EventHandler;
use raw_packet::RawPacket;
use types::Queues;
pub use types::{Ciphers, Direction, SharedState, State};

mod cipher;
mod conf;
mod parsable;
mod plugin;
mod plugins;
mod raw_packet;
mod types;
mod utils;

mod packet;
mod protocol;

pub use protocol::v754 as functions;

use crate::parsable::Parsable;

pub type DataQueue = deadqueue::unlimited::Queue<Vec<u8>>;

const SHUTDOWN_CHECK_TIMEOUT: u64 = 100;

// This function puts all received packets (in chunks of 4096 bytes) in the receiving queue.
async fn receiver(
    mut rx: OwnedReadHalf,
    queue: Arc<DataQueue>,
    socket_name: &str,
    is_closed: Arc<AtomicBool>,
) {
    let mut buf = [0; 4096];
    loop {
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
                // log::warn!("Did not receive new data in 60 seconds, assuming shutdown");
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
                        // log::warn!("Did not receive new data in 60 seconds, assuming shutdown");
                        // return;
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

// TODO: Add comments to this function
async fn parser(
    queues: Queues,
    shared_status: Arc<Mutex<SharedState>>,
    ciphers: Arc<Mutex<Ciphers>>,
    direction: Direction,
    is_closed: Arc<AtomicBool>,
    plugins: Arc<Mutex<Vec<Box<dyn EventHandler + Send>>>>,
) -> Result<(), ()> {
    let mut unprocessed_data = RawPacket::new();
    let functions = functions::get_functions();
    let config = conf::get_config();
    loop {
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

        let new_data = if direction == Direction::Clientbound {
            ciphers.lock().sp_cipher.decrypt(new_data)
        } else {
            new_data
        };

        unprocessed_data.push_vec(new_data);
        while unprocessed_data.len() > 0 {
            let o_data = unprocessed_data.get_vec();

            let packet_length = match unprocessed_data.decode_varint() {
                Ok(packet_length) => packet_length,
                Err(_) => {
                    unprocessed_data.set(o_data);
                    break;
                }
            };

            if (unprocessed_data.len() as i32) < packet_length {
                unprocessed_data.set(o_data);
                break;
            }

            let mut packet =
                raw_packet::RawPacket::from(unprocessed_data.read(packet_length as usize).unwrap());

            let mut original_packet = RawPacket::new();
            original_packet.encode_varint(packet_length);
            original_packet.push_vec(packet.get_vec());

            // Uncompress if needed
            if shared_status.lock().compress > 0 {
                let data_length = packet.decode_varint()?;
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

            let packet_id = packet.decode_varint()?;

            let mut not_processed = false;

            let func_id =
                match functions.get_name(&direction, &shared_status.lock().state, &packet_id) {
                    Some(func_name) => func_name,
                    None => {
                        not_processed = true;
                        &functions::Fid::Unparsable
                    }
                };

            let mut to_direction = direction;
            let mut out_data = original_packet.get_vec();

            let out_data = if not_processed {
                let out_data = if to_direction == Direction::Serverbound {
                    // Compress data if needed, then encrypt
                    ciphers.lock().ps_cipher.encrypt(out_data)
                } else {
                    out_data
                };

                out_data
            } else {
                let mut parsed_packet = match functions.get(func_id) {
                    Some(func) => func,
                    None => panic!("This should never happen, if it does: crash"),
                };

                let success = match parsed_packet.parse_packet(packet) {
                    Ok(_) => {
                        let packet_info = parsed_packet.get_printable();
                        if config.logging_packets.contains(&func_id.to_string())
                            || config.logging_packets.contains(&"*".to_string())
                        {
                            log::info!(
                                "{} [{}]{3:4$} {}",
                                direction.to_string().yellow(),
                                func_id.to_string().blue(),
                                packet_info,
                                "",
                                config.print_buffer - func_id.to_string().len()
                            );
                        }
                        true
                    }
                    Err(_) => {
                        log::error!("Could not parse packet!");
                        false
                    }
                };

                if success {
                    if parsed_packet.status_updating() {
                        parsed_packet
                            .update_status(&mut shared_status.lock())
                            .unwrap()
                    }
                    if parsed_packet.packet_editing() {
                        let shared_status_c = shared_status.lock().clone();
                        let mut shared_plugins = plugins.lock().clone();
                        match parsed_packet
                            .edit_packet(shared_status_c, &mut shared_plugins, &config)
                            .await
                        {
                            Ok((packet_vec, new_shared_status)) => {
                                if packet_vec.len() > 1 {
                                    for packet in packet_vec {
                                        let out_d = if new_shared_status.compress == 0 {
                                            packet.0.get_data_uncompressed().unwrap()
                                        } else {
                                            packet
                                                .0
                                                .get_data_compressed(
                                                    new_shared_status.compress as i32,
                                                )
                                                .unwrap()
                                        };
                                        match packet.1 {
                                            Direction::Serverbound => {
                                                let out_d = ciphers.lock().ps_cipher.encrypt(out_d);
                                                queues.proxy_server.push(out_d);
                                            }
                                            Direction::Clientbound => {
                                                queues.proxy_client.push(out_d)
                                            }
                                        }
                                    }
                                    out_data.clear();
                                } else if packet_vec.is_empty() {
                                } else {
                                    let packet = &packet_vec[0];
                                    to_direction = packet.1;
                                    out_data = if new_shared_status.compress == 0 {
                                        packet.0.get_data_uncompressed().unwrap()
                                    } else {
                                        packet
                                            .0
                                            .get_data_compressed(new_shared_status.compress as i32)
                                            .unwrap()
                                    };
                                }
                                shared_status.lock().set(new_shared_status.clone());
                                let mut locked_plugins = plugins.lock();
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
                    && parsed_packet.post_send_updating()
                    && parsed_packet
                        .post_send_update(&mut ciphers.lock(), &shared_status.lock())
                        .is_err()
                {
                    panic!("PSU failed, panicing.")
                }
                out_data
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
    let config = conf::get_config();
    let plugins = Arc::new(Mutex::new(plugins::get_plugins()));

    // Make a new  a new queue for all the directions to the proxy
    let queues = Queues {
        client_proxy: Arc::new(DataQueue::new()),
        proxy_client: Arc::new(DataQueue::new()),
        server_proxy: Arc::new(DataQueue::new()),
        proxy_server: Arc::new(DataQueue::new()),
    };

    // It then makes a shared state to share amongst all the threads
    let shared_ciphers: Arc<Mutex<Ciphers>> = Arc::new(Mutex::new(Ciphers::new()));

    // This part reads the sent data into a buffer
    let mut buffer = Vec::new();
    client_stream.read_buf(&mut buffer).await.unwrap();

    // It then tries to parse the first data into a packet.
    let mut first_data = RawPacket::from(buffer.clone());
    let packet_length = first_data.decode_varint()?;
    let mut raw_first_packet = RawPacket::from(first_data.read(packet_length as usize).unwrap());
    let packet_id = raw_first_packet.decode_varint()?;

    // If the packet ID is not 0, it is not a valid minecraft packet.
    if packet_id != 0 {
        log::error!("Packet ID did not match handshaking packet, terminating connection...");
        log::debug!("Packet: {:?}", buffer);
        return Ok(());
    };

    // It then continues to parse the packet
    let mut handshaking_packet = functions::serverbound::handshaking::Handshake::empty();
    if handshaking_packet.parse_packet(raw_first_packet).is_err() {
        log::error!("Invalid handshake packet! Closing connection...");
        return Err(());
    };

    // It then gets the IP address of the actual server
    let ip = handshaking_packet
        .server_address
        .strip_suffix(&config.domain_suffix)
        .unwrap()
        .to_string();

    // It looks if there is an SRV record present on the domain, if there is it uses that.
    log::debug!("Resolving ip: {:?}", ip);
    let resolver =
        TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let lookup = resolver.srv_lookup(format!("_minecraft._tcp.{}", ip)).await;

    let address = if lookup.is_ok() {
        let response = lookup.unwrap();
        let record = response.iter().next().unwrap();

        let ip = record.target().to_string().trim_matches('.').to_string();
        log::debug!("ip: {:?}", ip);

        record.target().to_string().trim_matches('.').to_string()
    } else {
        // Othwerise it just uses the IP directly
        ip.to_owned()
    };

    // It converts the data back to a packet, with all the stuff that's associated with that.
    let mut new_packet = RawPacket::new();
    new_packet.encode_varint(0);
    new_packet.encode_varint(handshaking_packet.protocol_version);
    new_packet.encode_string(address.clone());
    new_packet.encode_ushort(25565);
    new_packet.encode_varint(match handshaking_packet.next_state {
        State::Status => 1,
        State::Login => 2,
        _ => unreachable!(),
    });
    new_packet.prepend_length();
    // It adds the remaining data that was sent in the first packet, to make sure no data gets lost.
    new_packet.push_vec(first_data.get_vec());
    queues.client_proxy.push(new_packet.get_vec());

    // It connects to the server, for now the port 25565 is hardcoded.
    log::info!("Connecting to IP {}", address);
    let server_stream = match TcpStream::connect(&format!("{}:{}", address, 25565)).await {
        Ok(stream) => stream,
        Err(err) => {
            log::error!("Could not connect to ip: {}", err);
            return Ok(());
        }
    };
    log::info!("Connected...");

    // It creates a shared status where all data that is mutable or request specific is kept.
    let shared_status: Arc<Mutex<SharedState>> = Arc::new(Mutex::new(SharedState {
        access_token: config.player_auth_token,
        uuid: config.player_uuid,
        server_ip: address,
        user_ip,
        connection_id,
        ..SharedState::new()
    }));

    // It then splits both TCP streams up in rx and tx
    let (crx, ctx) = client_stream.into_split();
    let (srx, stx) = server_stream.into_split();
    let is_closed = Arc::new(AtomicBool::new(false));

    // It then starts multiple threads to put all the received data into the previously created queues
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

    // And it also starts two to put the send data in the tx's
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

    // It then starts a parser for both of the directions. It's a bit annoying to have to make so many clones but I can't think of a better way.
    tokio::spawn({
        let shared_status = shared_status.clone();
        let shared_ciphers = shared_ciphers.clone();
        let queues = queues.clone();
        let is_closed = is_closed.clone();
        let plugins = plugins.clone();
        async move {
            parser(
                queues,
                shared_status,
                shared_ciphers,
                Direction::Serverbound,
                is_closed,
                plugins,
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
            )
            .await
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load the logger, it has a fancy format with colours and it's spaced.
    // TODO: Add file logging
    Builder::from_default_env()
        .format(|buf, record| {
            let formatted_level = buf.default_styled_level(record.level());
            writeln!(buf, "{:<5} {}", formatted_level, record.args())
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    // Try to load config to make sure it works
    let config = conf::get_config();

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
        log::info!("Client connected, connection ID: {}...", next_connection_id);
        let ip = socket_addr.ip().to_string();
        log::info!("IP: {}", ip);
        // Start the client-handling thread (this will complete quickly)
        handle_connection(socket, ip, next_connection_id)
            .await
            .unwrap();
    }
}
