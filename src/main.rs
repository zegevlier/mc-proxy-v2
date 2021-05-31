#![allow(where_clauses_object_safety)]

use std::{io::Write, sync::Arc, time::Duration};

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

type DataQueue = deadqueue::unlimited::Queue<Vec<u8>>;

// This function puts all received packets (in chunks of 4096 bytes) in the receiving queue.
async fn receiver(mut rx: OwnedReadHalf, queue: Arc<DataQueue>, socket_name: &str) {
    let mut buf = [0; 4096];
    loop {
        let n = match timeout(Duration::from_secs(60), rx.read(&mut buf)).await {
            Ok(v) => match v {
                Ok(n) if n == 0 => {
                    log::warn!("Socket closed: {}", socket_name);
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    log::error!("Failed to read from socket: {}", e);
                    return;
                }
            },
            Err(_) => {
                log::warn!("Did not receive new data in 60 seconds, assuming shutdown");
                return;
            }
        };
        queue.push(buf[0..n].to_vec());
    }
}

// This sends the data in the respective queues to the tx.
async fn sender(mut tx: OwnedWriteHalf, queue: Arc<DataQueue>) {
    loop {
        if let Err(e) = tx
            .write_all(
                &(match timeout(Duration::from_secs(60), queue.pop()).await {
                    Ok(b) => b,
                    Err(_) => {
                        log::warn!("Did not receive new data in 60 seconds, assuming shutdown");
                        return;
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
    client_proxy_queue: Arc<DataQueue>,
    server_proxy_queue: Arc<DataQueue>,
    proxy_client_queue: Arc<DataQueue>,
    proxy_server_queue: Arc<DataQueue>,
    shared_status: Arc<Mutex<SharedState>>,
    ciphers: Arc<Mutex<Ciphers>>,
    direction: Direction,
) -> Result<(), ()> {
    let mut unprocessed_data = RawPacket::new();
    let functions = functions::get_functions();
    let mut plugins = plugins::get_plugins();
    let config = conf::get_config();
    loop {
        let new_data = match timeout(
            Duration::from_secs(60),
            match direction {
                Direction::Serverbound => client_proxy_queue.pop(),
                Direction::Clientbound => server_proxy_queue.pop(),
            },
        )
        .await
        {
            Ok(new_data) => new_data,
            Err(_) => {
                log::warn!("Did not receive new data in 60 seconds, assuming shutdown");
                break;
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
                                22 - func_id.to_string().len()
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
                        match parsed_packet
                            .edit_packet(shared_status_c, &mut plugins)
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
                                                proxy_server_queue.push(out_d);
                                            }
                                            Direction::Clientbound => {
                                                proxy_client_queue.push(out_d)
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
                Direction::Serverbound => proxy_server_queue.push(out_data),
                Direction::Clientbound => proxy_client_queue.push(out_data),
            }
        }
    }
    Ok(())
}

async fn handle_connection(mut client_stream: TcpStream) -> Result<(), ()> {
    // Make a new  a new queue for all the directions to the proxy
    let client_proxy_queue = Arc::new(DataQueue::new());
    let proxy_client_queue = Arc::new(DataQueue::new());
    let server_proxy_queue = Arc::new(DataQueue::new());
    let proxy_server_queue = Arc::new(DataQueue::new());
    // It then makes a shared state to share amongst all the threads
    let shared_status: Arc<Mutex<SharedState>> = Arc::new(Mutex::new(SharedState::new()));
    let shared_ciphers: Arc<Mutex<Ciphers>> = Arc::new(Mutex::new(Ciphers::new()));

    let mut buffer = Vec::new();
    client_stream.read_buf(&mut buffer).await.unwrap();
    let mut raw_first_packet = RawPacket::from(buffer.clone());
    let _packet_length = raw_first_packet.decode_varint()?;
    let packet_id = raw_first_packet.decode_varint()?;
    if packet_id != 0 {
        log::error!("Packet ID did not match handshaking packet, terminating connection...");
        log::debug!("Packet: {:?}", buffer);
        return Ok(());
    };
    let protocol_version = raw_first_packet.decode_varint()?;
    let server_address = &raw_first_packet.decode_string()?;
    let _server_port = raw_first_packet.decode_ushort()?;
    let next_state = raw_first_packet.decode_varint()?;

    let ip = server_address.strip_suffix(".proxy").unwrap().to_string();

    // It connects to the new IP, if it fails just error.
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
        ip.to_owned()
    };

    let mut new_packet = RawPacket::new();
    new_packet.encode_varint(0);
    new_packet.encode_varint(protocol_version);
    new_packet.encode_string(address.clone());
    new_packet.encode_ushort(25565);
    new_packet.encode_varint(next_state);
    new_packet.prepend_length();
    new_packet.push_vec(raw_first_packet.get_vec());
    client_proxy_queue.push(new_packet.get_vec());

    log::info!("Connecting to IP {}", address);
    let server_stream = match TcpStream::connect(&format!("{}:{}", address, 25565)).await {
        Ok(stream) => stream,
        Err(err) => {
            log::error!("Could nto connect ot ip: {}", err);
            return Ok(());
        }
    };
    log::info!("Connected...");

    // It then splits both TCP streams up in rx and tx
    let (crx, ctx) = client_stream.into_split();
    let (srx, stx) = server_stream.into_split();

    // It then starts multiple threads to put all the received data into the previously created queues
    tokio::spawn({
        let client_proxy_queue = client_proxy_queue.clone();
        async move { receiver(crx, client_proxy_queue, "client").await }
    });
    tokio::spawn({
        let server_proxy_queue = server_proxy_queue.clone();
        async move { receiver(srx, server_proxy_queue, "server").await }
    });

    // And it also starts two to put the send data in the tx's
    tokio::spawn({
        let proxy_client_queue = proxy_client_queue.clone();
        async move { sender(ctx, proxy_client_queue).await }
    });
    tokio::spawn({
        let proxy_server_queue = proxy_server_queue.clone();
        async move { sender(stx, proxy_server_queue).await }
    });

    // It then starts a parser for both of the directions. It's a bit annoying to have to make so many clones but I can't think of a better way.
    tokio::spawn({
        let shared_status = shared_status.clone();
        let shared_ciphers = shared_ciphers.clone();
        let client_proxy_queue = client_proxy_queue.clone();
        let server_proxy_queue = server_proxy_queue.clone();
        let proxy_client_queue = proxy_client_queue.clone();
        let proxy_server_queue = proxy_server_queue.clone();
        async move {
            parser(
                client_proxy_queue,
                server_proxy_queue,
                proxy_client_queue,
                proxy_server_queue,
                shared_status,
                shared_ciphers,
                Direction::Serverbound,
            )
            .await
        }
    });

    tokio::spawn({
        async move {
            parser(
                client_proxy_queue,
                server_proxy_queue,
                proxy_client_queue,
                proxy_server_queue,
                shared_status,
                shared_ciphers,
                Direction::Clientbound,
            )
            .await
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load the logger, it has a fancy format with colours and it's spaced.
    // TODO: Add  file logging
    Builder::from_default_env()
        .format(|buf, record| {
            let formatted_level = buf.default_styled_level(record.level());
            writeln!(buf, "{:<5} {}", formatted_level, record.args())
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    // Try to load config to make sure it works
    conf::get_config();

    log::info!("Starting listener...");
    // Start listening on the ip waiting for new connections
    let mc_client_listener = match TcpListener::bind("127.0.0.1:25555").await {
        Ok(listener) => listener,
        Err(err) => panic!("Could not connect to server: {}", err),
    };

    loop {
        // If this continues, a new client is connected.
        let (socket, _) = mc_client_listener.accept().await?;
        log::info!("Client connected...");
        // Start the client-handling thread (this will complete quickly)
        handle_connection(socket).await.unwrap();
    }
}
