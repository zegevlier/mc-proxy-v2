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

        if message.starts_with('.') {
            if message == ".test" {
                return_packet_vec.push((
                    generate_message_packet("Test packet received!").unwrap(),
                    Direction::Clientbound,
                ));
            } else if message.starts_with(".say ") {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_string(message.strip_prefix(".say ").unwrap().to_string())?;
                return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
            } else if message.starts_with(".o ") {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_string("en_us".to_string())?;
                raw_packet.encode_byte(15)?;
                raw_packet.encode_varint(0)?;
                raw_packet.encode_bool(true)?;

                if message.contains("off") {
                    raw_packet.encode_ubyte(65)?;
                } else {
                    raw_packet.encode_ubyte(127)?;
                }
                raw_packet.encode_varint(1)?;

                return_packet_vec.push((Packet::from(raw_packet, 0x05), Direction::Serverbound));
            } else {
                return_packet_vec.push((
                    generate_message_packet("Command not found!").unwrap(),
                    Direction::Clientbound,
                ));
            }
        } else {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_string(message.to_string())?;
            return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
        }

        Ok((return_packet_vec, status))
    }
}
