use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};

fn generate_message_packet(text: &str) -> Result<Packet, ()> {
    let mut raw_packet = RawPacket::new();
    raw_packet.encode_string(format!(
        "{{\"extra\":[{{\"color\":\"red\",\"text\":\"proxy\"}},{{\"text\":\"> {}\"}}],\"text\":\"\"}}",
        text
    ))?;
    raw_packet.encode_byte(1)?;
    raw_packet.encode_uuid(0)?;
    Ok(Packet::from(raw_packet, 0x0E))
}

#[derive(Clone)]
pub struct ChatMessageServerbound {
    message: String,
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
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
        let mut return_packet_vec = Vec::new();
        let message = &self.message.clone();
        let mut raw_packet = RawPacket::new();
        raw_packet.encode_string(message.to_string())?;

        if self.message.starts_with('.') {
            if message == ".test" {
                return_packet_vec.push((
                    generate_message_packet("Test packet recieved!").unwrap(),
                    Direction::Clientbound,
                ));
            }
        } else {
            return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
        }

        Ok((return_packet_vec, status))
    }
}
//
