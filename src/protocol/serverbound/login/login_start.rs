use crate::{functions::fid_to_pid, parsable::Parsable, Direction, SharedState};

use packet::{Packet, RawPacket, SafeDefault};

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
    authentication_token: Option<String>,
    uuid: Option<String>,
    allowed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    username: String,
    mc_server_address: String,
    login_ip: String,
}

#[derive(Clone, Serialize)]
pub struct LoginStart {
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthSubResponse {
    success: bool,
    message: Option<String>,
}

#[async_trait::async_trait]
impl Parsable for LoginStart {
    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn crate::EventHandler + Send>>,
        config: &crate::conf::Configuration,
    ) -> Result<Vec<(packet::Packet, crate::Direction)>, ()> {
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

                        return Ok(vec![(
                            Packet::from(new_packet, fid_to_pid(crate::functions::Fid::Disconnect)),
                            Direction::Clientbound,
                        )]);
                    }
                };
            log::info!("Connection to websocket established.");

            let message_data = serde_json::to_string(&AuthRequest {
                login_ip: status.user_ip.clone(),
                mc_server_address: status.server_ip.clone(),
                username: self.username.clone(),
            })
            .unwrap();

            log::debug!("{}", message_data);

            tokio::time::sleep_until(
                tokio::time::Instant::now() + std::time::Duration::from_millis(100),
            )
            .await;
            ws.send(Message::text(&message_data)).await.unwrap();

            // ws.send(Message::text("Hi!")).await.unwrap();

            log::info!("Sent authentication request!");

            match serde_json::from_str::<AuthSubResponse>(
                dbg! {ws.next().await.unwrap().unwrap().to_text().unwrap()},
            )
            .unwrap()
            .success
            {
                true => {}
                false => {
                    log::error!("No client found listening for that name");
                    let mut new_packet = RawPacket::new();
                    new_packet.encode(&"{\"text\":\"Failed to authenticate\"}".to_string())?;

                    return Ok(vec![(
                        Packet::from(new_packet, fid_to_pid(crate::functions::Fid::Disconnect)),
                        Direction::Clientbound,
                    )]);
                }
            };

            let return_msg = match ws.next().await.unwrap() {
                Ok(msg) => msg,
                Err(_) => {
                    let mut new_packet = RawPacket::new();
                    new_packet.encode(&"{\"text\":\"Failed to authenticate\"}".to_string())?;

                    return Ok(vec![(
                        Packet::from(new_packet, fid_to_pid(crate::functions::Fid::Disconnect)),
                        Direction::Clientbound,
                    )]);
                }
            };

            let parsed_return_msg: AuthResponse =
                serde_json::from_str(return_msg.to_text().unwrap()).unwrap();

            println!("{:?}", parsed_return_msg);

            if parsed_return_msg.allowed {
                status.access_token = parsed_return_msg.authentication_token.unwrap();
                status.uuid = parsed_return_msg.uuid.unwrap();
                return Ok(vec![]);
            } else {
                log::error!("Connection disallowed!");
                let mut new_packet = RawPacket::new();
                new_packet.encode(&"{\"text\":\"Failed to authenticate\"}".to_string())?;

                return Ok(vec![(
                    Packet::from(new_packet, fid_to_pid(crate::functions::Fid::Disconnect)),
                    Direction::Clientbound,
                )]);
                // let mut new_packet = RawPacket::new();
                // new_packet.encode_string(
                //     "{\"text\":\"Connection was disallowed! How dare you...\"}".to_string(),
                // );

                // return Ok(vec![(
                //     Packet::from(new_packet, fid_to_pid(crate::functions::Fid::Disconnect)),
                //     Direction::Clientbound,
                // )]);
            }
        } else {
            // Just send the packet to the client
            Ok(vec![])
        }
    }
}

impl std::fmt::Display for LoginStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

impl SafeDefault for LoginStart {
    fn default() -> Self {
        Self {
            username: String::new(),
        }
    }
}

impl packet::ProtoDec for LoginStart {
    fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
        self.username = p.decode()?;
        Ok(())
    }
}

impl packet::ProtoEnc for LoginStart {
    fn encode(&self, p: &mut RawPacket) -> packet::Result<()> {
        p.encode(&self.username)?;
        Ok(())
    }
}
