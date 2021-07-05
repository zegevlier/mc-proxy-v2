use crate::{
    functions::fid_to_pid, packet::Packet, parsable::Parsable, raw_packet::RawPacket, Direction,
};

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
    authentication_token: String,
    uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    username: String,
    mc_server_address: String,
    login_ip: String,
}

#[derive(Clone)]
pub struct LoginStart {
    username: String,
}

#[async_trait::async_trait]
impl Parsable for LoginStart {
    fn empty() -> Self {
        Self {
            username: "".into(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.username = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        self.username.to_string()
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: crate::SharedState,
        _plugins: &mut Vec<Box<dyn crate::EventHandler + Send>>,
        config: &crate::conf::Configuration,
    ) -> Result<
        (
            Vec<(crate::packet::Packet, crate::Direction)>,
            crate::SharedState,
        ),
        (),
    > {
        let mut status = status;
        if config.ws_enabled {
            let (mut ws, _) = connect_async(&config.ws_url)
                .await
                .expect("Failed to connect to websocket.");

            ws.send(Message::text(format!("${}", config.ws_secret)))
                .await
                .unwrap();
            ws.send(Message::text(
                &serde_json::to_string(&AuthRequest {
                    login_ip: status.user_ip.clone(),
                    mc_server_address: status.server_ip.clone(),
                    username: self.username.clone(),
                })
                .unwrap(),
            ))
            .await
            .unwrap();
            match ws.next().await.unwrap().unwrap().to_text().unwrap() {
                "OK" => {}
                "ERR: NOT_FOUND" => {
                    log::error!("No client found listening for that name");
                    let mut new_packet = RawPacket::new();
                    new_packet.encode_string("{\"text\":\"Failed to authenticate\"}".to_string());

                    return Ok((
                        vec![(
                            Packet::from(new_packet, fid_to_pid(crate::functions::Fid::Disconnect)),
                            Direction::Clientbound,
                        )],
                        status,
                    ));
                }
                _ => unreachable!(),
            };

            let return_msg = ws.next().await.unwrap().unwrap();
            let parsed_return_msg: AuthResponse =
                serde_json::from_str(&return_msg.to_text().unwrap()).unwrap();

            status.access_token = parsed_return_msg.authentication_token;
            status.uuid = parsed_return_msg.uuid;
        }

        Ok((vec![], status))
    }
}
