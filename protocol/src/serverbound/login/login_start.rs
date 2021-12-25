use crate::packet;
use mcore::types::Direction;
use packet::SharedState;

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    username: String,
    mc_server_address: String,
    login_ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
    authentication_token: Option<String>,
    uuid: Option<String>,
    allowed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthSubResponse {
    success: bool,
    message: Option<String>,
}

packet! {
    LoginStart, all,
    {
        username: String,
    }
}

#[async_trait::async_trait]
impl Parsable for LoginStart {
    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn plugin::EventHandler + Send>>,
        config: &config_loader::Configuration,
    ) -> packet::Result<Option<Vec<(packet::Packet, mcore::types::Direction)>>> {
        let mut status = status;
        if config.ws_enabled {
            let (mut ws, _) =
                match connect_async(format!("{}/{}", &config.ws_url, &config.ws_secret)).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        log::error!("{}", e);
                        let mut new_packet = RawPacket::new();
                        new_packet.encode(
                            &"{\"text\":\"WS server down! Please report this!\"}".to_string(),
                        )?;

                        return Ok(Some(vec![(
                            Packet::from(
                                new_packet,
                                crate::current_protocol::fid_to_pid(crate::Fid::Disconnect),
                            ),
                            Direction::Clientbound,
                        )]));
                    }
                };
            log::info!("Connection to websocket established.");

            let message_data = serde_json::to_string(&AuthRequest {
                login_ip: status.user_ip.clone(),
                mc_server_address: status.server_ip.clone(),
                username: self.username.clone(),
            })
            .unwrap();

            tokio::time::sleep_until(
                tokio::time::Instant::now() + std::time::Duration::from_millis(100),
            )
            .await;
            ws.send(Message::text(&message_data)).await.unwrap();

            log::info!("Sent authentication request!");

            match serde_json::from_str::<AuthSubResponse>(
                ws.next().await.unwrap().unwrap().to_text().unwrap(),
            )
            .unwrap()
            .success
            {
                true => {}
                false => {
                    log::error!("No client found listening for that name");
                    let mut new_packet = RawPacket::new();
                    new_packet.encode(&"{\"text\":\"Failed to authenticate\"}".to_string())?;

                    return Ok(Some(vec![(
                        Packet::from(
                            new_packet,
                            crate::current_protocol::fid_to_pid(crate::Fid::Disconnect),
                        ),
                        Direction::Clientbound,
                    )]));
                }
            };

            let return_msg = match ws.next().await.unwrap() {
                Ok(msg) => msg,
                Err(_) => {
                    let mut new_packet = RawPacket::new();
                    new_packet.encode(&"{\"text\":\"Failed to authenticate\"}".to_string())?;

                    return Ok(Some(vec![(
                        Packet::from(
                            new_packet,
                            crate::current_protocol::fid_to_pid(crate::Fid::Disconnect),
                        ),
                        Direction::Clientbound,
                    )]));
                }
            };

            let parsed_return_msg: AuthResponse =
                serde_json::from_str(return_msg.to_text().unwrap()).unwrap();

            if parsed_return_msg.allowed {
                status.access_token = parsed_return_msg.authentication_token.unwrap();
                status.uuid = parsed_return_msg.uuid.unwrap();
                return Ok(Some(vec![]));
            } else {
                log::error!("Connection disallowed!");
                let mut new_packet = RawPacket::new();
                new_packet.encode(&"{\"text\":\"Failed to authenticate\"}".to_string())?;

                return Ok(Some(vec![(
                    Packet::from(
                        new_packet,
                        crate::current_protocol::fid_to_pid(crate::Fid::Disconnect),
                    ),
                    Direction::Clientbound,
                )]));
            }
        } else {
            // Just send the packet to the client
            Ok(Some(vec![]))
        }
    }
}