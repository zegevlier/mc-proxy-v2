//, rainbowfy};
use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};

#[derive(Clone)]
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
        status: SharedState,
        plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
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
                Packet::from(raw_packet, 0x03),
                Direction::Serverbound,
            )]);
        }

        Ok((return_vec.unwrap(), status))
    }
}
