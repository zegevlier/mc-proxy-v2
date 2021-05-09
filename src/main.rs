#![allow(where_clauses_object_safety)]
use miniz_oxide::inflate::decompress_to_vec_zlib;
use parking_lot::Mutex;
use std::{io::Write, sync::Arc, time::Duration};

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

mod cipher;
mod packet;
mod parsable;
mod types;
mod utils;

mod clientbound;
mod functions;
mod serverbound;

use packet::Packet;
pub use types::{Direction, SharedState, State};

type DataQueue = deadqueue::unlimited::Queue<Vec<u8>>;

// This function puts all recieved packets (in chunks of 4096 bytes) in the recieving queue.
async fn reciever(mut rx: OwnedReadHalf, queue: Arc<DataQueue>) {
    let mut buf = [0; 4096];
    loop {
        let n = match timeout(Duration::from_secs(60), rx.read(&mut buf)).await {
            Ok(v) => match v {
                Ok(n) if n == 0 => {
                    log::warn!("Socket closed");
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    log::error!("Failed to read from socket: {}", e);
                    return;
                }
            },
            Err(_) => {
                log::warn!("Did not recieve new data in 60 seconds, assuming shutdown");
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
                        log::warn!("Did not recieve new data in 60 seconds, assuming shutdown");
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
    direction: Direction,
) -> Result<(), ()> {
    let mut unprocessed_data = Packet::new();
    let functions = functions::get_functions();
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
                log::warn!("Did not recieve new data in 60 seconds, assuming shutdown");
                break;
            }
        };

        let new_data = if direction == Direction::Clientbound {
            shared_status.lock().sp_cipher.decrypt(new_data)
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
                packet::Packet::from(unprocessed_data.read(packet_length as usize).unwrap());

            let mut original_packet = Packet::new();
            original_packet.encode_varint(packet_length)?;
            original_packet.push_vec(packet.get_vec());

            if direction == Direction::Clientbound {
                // Uncompress if needed
                if shared_status.lock().compress > 0 {
                    let data_length = packet.decode_varint()?;
                    if data_length > 0 {
                        let decompressed_packet = match decompress_to_vec_zlib(&packet.get_vec()) {
                            Ok(decompressed_packet) => decompressed_packet,
                            Err(why) => {
                                log::error!("Decompress error: {:?}", why);
                                break;
                            }
                        };
                        packet.set(decompressed_packet);
                    }
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
                    shared_status.lock().ps_cipher.encrypt(out_data)
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
                        log::info!(
                            "{} [{}]{3:4$} {}",
                            direction.to_string().yellow(),
                            func_id.to_string().blue(),
                            packet_info,
                            "",
                            20 - func_id.to_string().len()
                        );
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
                        match parsed_packet.edit_packet(shared_status_c).await {
                            Ok((packet, new_direction, new_shared_status)) => {
                                to_direction = new_direction;
                                out_data = packet.get_vec();
                                shared_status.lock().set(new_shared_status.clone());
                            }
                            Err(_) => {
                                panic!("This should never happen");
                            }
                        };
                    }
                }

                if to_direction == Direction::Serverbound {
                    //TODO Add data compression, but this needs to be done with the packet type.
                    out_data = shared_status.lock().ps_cipher.encrypt(out_data)
                }
                if success && parsed_packet.post_send_updating() {
                    match parsed_packet.post_send_update(&mut shared_status.lock()) {
                        Ok(_) => {
                            log::debug!("Ran post send update")
                        }
                        Err(_) => {
                            panic!("This should never happen")
                        }
                    };
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

async fn handle_connection(client_stream: TcpStream) -> std::io::Result<()> {
    // Make a new  a new queue for all the directions to the proxy
    let client_proxy_queue = Arc::new(DataQueue::new());
    let proxy_client_queue = Arc::new(DataQueue::new());
    let server_proxy_queue = Arc::new(DataQueue::new());
    let proxy_server_queue = Arc::new(DataQueue::new());
    // It then makes a shared state to share amongst all the threads
    let shared_status: Arc<Mutex<SharedState>> = Arc::new(Mutex::new(SharedState::new()));

    // It connects to the new IP, if it fails just error.
    let ip = "127.0.0.1:25565";
    log::info!("Connecting to IP {}", ip);
    let server_stream = TcpStream::connect(ip).await?;

    // It then splits both TCP streams up in rx and tx
    let (srx, stx) = server_stream.into_split();
    let (crx, ctx) = client_stream.into_split();

    // It then starts multiple threads to put all the recieved data into the previously created queues
    tokio::spawn({
        let client_proxy_queue = client_proxy_queue.clone();
        async move { reciever(crx, client_proxy_queue).await }
    });
    tokio::spawn({
        let server_proxy_queue = server_proxy_queue.clone();
        async move { reciever(srx, server_proxy_queue).await }
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
        // Start the client-handeling thread (this will complete quickly)
        handle_connection(socket).await?;
    }
}
