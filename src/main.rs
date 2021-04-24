#![allow(where_clauses_object_safety)]
use miniz_oxide::inflate::decompress_to_vec_zlib;
use parking_lot::Mutex;
use std::{io::Write, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
};

use colored::*;
use env_logger::Builder;
use log::LevelFilter;

mod cipher;
mod packet;
mod types;
mod utils;

mod clientbound;
mod functions;
mod serverbound;

use packet::Packet;
pub use types::{Direction, SharedState, State};

type DataQueue = deadqueue::unlimited::Queue<Vec<u8>>;

async fn reciever(mut rx: OwnedReadHalf, queue: Arc<DataQueue>) {
    let mut buf = [0; 4096];
    loop {
        let n = match rx.read(&mut buf).await {
            Ok(n) if n == 0 => {
                log::warn!("Socket closed");
                return;
            }
            Ok(n) => n,
            Err(e) => {
                log::error!("Failed to read from socket: {}", e);
                return;
            }
        };
        queue.push(buf[0..n].to_vec());
    }
}

async fn sender(mut tx: OwnedWriteHalf, queue: Arc<DataQueue>) {
    loop {
        if let Err(e) = tx.write_all(&queue.pop().await).await {
            log::error!("Failed to write to socket: {}", e);
        };
    }
}

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
        // if let Some(copied_state) = shared_status.lock().ps_cipher.encryptor.clone() {
        //     log::debug!("{:?}", copied_state.iv);
        // };

        let new_data = match direction {
            Direction::Serverbound => client_proxy_queue.pop().await,
            Direction::Clientbound => server_proxy_queue.pop().await,
        };
        // log::debug!("{}", direction.to_string());

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
            // log::debug!("new loop");
            // log::debug!("{}", packet_length);

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

            // log::debug!("{}", packet_id);
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

            // let mut shared_status_c = shared_status.lock().clone();
            let out_data = if !not_processed {
                let mut parsed_packet = match functions.get(func_id) {
                    Some(func) => func.clone(),
                    None => panic!("Oof"),
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
                                shared_status.lock().set(new_shared_status);
                            }
                            Err(_) => {}
                        };
                    }
                }

                if to_direction == Direction::Serverbound {
                    // Compress data if needed
                    out_data = shared_status.lock().ps_cipher.encrypt(out_data)
                }
                // let mut shared_status_c = shared_status.lock().clone();
                if success {
                    if parsed_packet.post_send_updating() {
                        println!("{:?}", shared_status.lock().ps_cipher.encryptor.is_some());
                        match parsed_packet.post_send_update(&mut shared_status.lock()) {
                            Ok(_) => {
                                log::debug!("Ran post send update")
                            }
                            Err(_) => {}
                        };
                    }
                }
                println!("{:?}", shared_status.lock().ps_cipher.encryptor.is_some());
                out_data
            } else {
                let out_data = if to_direction == Direction::Serverbound {
                    println!("{:?}", out_data);
                    // Compress data if needed
                    println!("{:?}", shared_status.lock().ps_cipher.encryptor.is_some());
                    shared_status.lock().ps_cipher.encrypt(out_data)
                } else {
                    out_data
                };
                // shared_status.lock().set(shared_status_c);
                if direction == Direction::Serverbound {
                    println!("{:?}", out_data);
                }

                out_data
            };
            // shared_status.lock().set(shared_status_c);

            match to_direction {
                Direction::Serverbound => proxy_server_queue.push(out_data),
                Direction::Clientbound => proxy_client_queue.push(out_data),
            }
        }
        // let out_data = unprocessed_data.read(unprocessed_data.len())?;
    }
}

async fn handle_connection(client_stream: TcpStream) -> std::io::Result<()> {
    let client_proxy_queue = Arc::new(DataQueue::new());
    let proxy_client_queue = Arc::new(DataQueue::new());
    let server_proxy_queue = Arc::new(DataQueue::new());
    let proxy_server_queue = Arc::new(DataQueue::new());
    let shared_status: Arc<Mutex<SharedState>> = Arc::new(Mutex::new(SharedState::new()));

    let server_stream = TcpStream::connect("127.0.0.1:25565").await?;

    let (srx, stx) = server_stream.into_split();
    let (crx, ctx) = client_stream.into_split();

    let cp_queue = client_proxy_queue.clone();
    tokio::spawn(async move { reciever(crx, cp_queue).await });
    let sp_queue = server_proxy_queue.clone();
    tokio::spawn(async move { reciever(srx, sp_queue).await });

    let pc_queue = proxy_client_queue.clone();
    tokio::spawn(async move { sender(ctx, pc_queue).await });
    let ps_queue = proxy_server_queue.clone();
    tokio::spawn(async move { sender(stx, ps_queue).await });

    let sb_shared_status = shared_status.clone();
    let sb_cp_queue = client_proxy_queue.clone();
    let sb_sp_queue = server_proxy_queue.clone();
    let sb_pc_queue = proxy_client_queue.clone();
    let sb_ps_queue = proxy_server_queue.clone();
    tokio::spawn(async move {
        parser(
            sb_cp_queue,
            sb_sp_queue,
            sb_pc_queue,
            sb_ps_queue,
            sb_shared_status,
            Direction::Serverbound,
        )
        .await
    });

    let cb_shared_status = shared_status.clone();
    let cb_cp_queue = client_proxy_queue.clone();
    let cb_sp_queue = server_proxy_queue.clone();
    let cb_pc_queue = proxy_client_queue.clone();
    let cb_ps_queue = proxy_server_queue.clone();
    tokio::spawn(async move {
        parser(
            cb_cp_queue,
            cb_sp_queue,
            cb_pc_queue,
            cb_ps_queue,
            cb_shared_status,
            Direction::Clientbound,
        )
        .await
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

    log::info!("Starting listener...");
    // Start listening on `BIND_ADDRESS` for new connections
    let mc_client_listener = TcpListener::bind("127.0.0.1:25555").await?;

    loop {
        // If this continues, a new client is connected.
        let (socket, _) = mc_client_listener.accept().await?;
        log::info!("Client connected...");
        // Start the client-handeling thread (this will complete quickly)
        handle_connection(socket).await?;
    }
}
