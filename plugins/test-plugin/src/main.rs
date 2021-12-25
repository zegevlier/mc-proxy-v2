use std::collections::HashMap;

use packet::RawPacket;
use tonic::{transport::Server, Request, Response, Status};

use plugin_interface::plugin_server::{Plugin, PluginServer};
use plugin_interface::ClientSendMessage;

fn get_emoji_hashmap() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("eyes".to_string(), "媃".to_string());
    map
}

#[derive(Debug, Default)]
pub struct TestPlugin {}

#[tonic::async_trait]
impl Plugin for TestPlugin {
    async fn on_client_send_message(
        &self,
        request: Request<ClientSendMessage>,
    ) -> Result<Response<plugin_interface::PluginResponse>, Status> {
        let request = request.into_inner();
        if request.message.starts_with('.') {
            // Decode two numbers from the request.message
            let mut split = request.message.split(' ');
            let chunk_x = split
                .next()
                .unwrap()
                .trim_start_matches('.')
                .parse::<i32>()
                .unwrap();
            let chunk_y = split
                .next()
                .unwrap()
                .trim_start_matches('.')
                .parse::<i32>()
                .unwrap();

            let mut return_packet = RawPacket::new();
            return_packet.encode(&chunk_x).unwrap();
            return_packet.encode(&chunk_y).unwrap();

            let reply = plugin_interface::PluginResponse {
                next: false,
                original: false,
                packets: vec![plugin_interface::Packet {
                    data: return_packet.get_vec(),
                    pid: 0x1C,
                    direction: 1,
                }],
            };
            return Ok(Response::new(reply));
        } else if request.message.contains(':') {
            let split = request.message.split(':');
            let emoji_map = get_emoji_hashmap();
            let mut return_message = "".to_string();
            for emoji in split.into_iter() {
                if let Some(emoji_value) = emoji_map.get(emoji) {
                    return_message.push_str(emoji_value);
                } else {
                    return_message.push_str(emoji);
                }
            }
            let mut return_packet = RawPacket::new();
            return_packet.encode(&return_message).unwrap();
            return Ok(Response::new(plugin_interface::PluginResponse {
                next: false,
                original: false,
                packets: vec![plugin_interface::Packet {
                    data: return_packet.get_vec(),
                    pid: 0x03,
                    direction: 0,
                }],
            }));

        };
        Ok(Response::new(plugin_interface::PluginResponse {
            next: true,
            original: true,
            packets: vec![],
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = TestPlugin::default();

    Server::builder()
        .add_service(PluginServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
