//, rainbowfy};
use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
    packet::Packet,
    parsable::Parsable,
    raw_packet::RawPacket,
};
use crate::{Direction, SharedState};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ChatMessageServerbound {
    pub message: String,
}

#[async_trait::async_trait]
impl Parsable for ChatMessageServerbound {
    fn empty() -> Self {
        Self {
            message: String::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.message = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        self.message.clone()
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        _status: &mut SharedState,
        plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        let mut return_vec = None;
        for plugin in plugins {
            match plugin.on_message(self) {
                Some(plugin_vec) => {
                    return_vec = Some(plugin_vec);
                    break;
                }
                None => continue,
            }
        }
        if return_vec.is_none() {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_string(self.message.to_string());
            return_vec = Some(vec![(
                Packet::from(raw_packet, fid_to_pid(Fid::ChatMessageServerbound)),
                Direction::Serverbound,
            )]);
        }

        Ok(return_vec.unwrap())
    }
}
