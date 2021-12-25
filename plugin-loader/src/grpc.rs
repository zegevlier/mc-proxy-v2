#![allow(where_clauses_object_safety)]

use std::sync::{Arc, Mutex};

use tonic::transport::Channel;
use tower::timeout::Timeout;

use mcore::types::Direction;
use packet::{Chat, Packet, RawPacket, Varint};
use plugin_interface::{plugin_client::PluginClient, ClientSendMessage};

#[derive(Clone)]
pub struct Grcp {
    plugins: Vec<Arc<Mutex<PluginClient<Timeout<Channel>>>>>,
}

#[async_trait::async_trait]
impl plugin::EventHandler for Grcp {
    async fn new() -> Self {
        let channel = match Channel::from_static("http://[::1]:50051").connect().await {
            Ok(channel) => channel,
            Err(e) => {
                println!("Failed to connect to plugin server: {}", e);
                return Grcp { plugins: vec![] };
            }
        };
        let timeout_channel = Timeout::new(channel, std::time::Duration::from_millis(50));
        let plugin = PluginClient::new(timeout_channel);
        Self {
            plugins: vec![Arc::new(Mutex::new(plugin))],
        }
    }

    async fn on_message(&mut self, message: &Chat) -> Option<plugin::PluginReponse> {
        for plugin in self.plugins.iter_mut() {
            let mut client = plugin.lock().unwrap().clone();
            let request = tonic::Request::new(ClientSendMessage {
                message: message.get_string().into(),
            });

            let response = match client.on_client_send_message(request).await {
                Ok(response) => response,
                Err(e) => {
                    log::debug!("Error: {:?}", e);
                    continue;
                }
            };

            let response = response.into_inner();
            if response.next {
                continue;
            }

            return Some(plugin::PluginReponse {
                send_original: response.original,
                packets: response
                    .packets
                    .into_iter()
                    .map(|packet: plugin_interface::Packet| {
                        (
                            Packet::from(
                                RawPacket::from(packet.data.clone()),
                                Varint::from(packet.pid),
                            ),
                            match packet.direction {
                                0 => Direction::Serverbound,
                                1 => Direction::Clientbound,
                                _ => unreachable!(),
                            },
                        )
                    })
                    .collect::<Vec<(packet::Packet, Direction)>>(),
            });
        }
        None
    }
}
