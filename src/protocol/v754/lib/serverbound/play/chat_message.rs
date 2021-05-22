use crate::utils::generate_message_packet;
use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};

#[derive(Clone)]
pub struct ChatMessageServerbound {
    message: String,
}

fn rainbowfy(message: String) -> String {
    let mut return_message = String::new();
    let rainbow_characters = "c6eab5";
    for (i, cha) in message.chars().enumerate() {
        match cha {
            ' ' => return_message.push(cha),
            _ => {
                return_message.push('&');
                return_message.push(
                    rainbow_characters
                        .chars()
                        .nth(i % rainbow_characters.len())
                        .unwrap(),
                );
                return_message.push(cha);
            }
        }
    }
    return_message
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
                raw_packet.encode_string(message.strip_prefix(".say ").unwrap().to_string());
                return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
            } else if message.starts_with(".rb ") {
                let mut raw_packet = RawPacket::new();
                raw_packet
                    .encode_string(rainbowfy(message.strip_prefix(".rb ").unwrap().to_string()));
                return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
            } else if message.starts_with(".rby ") {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_string(
                    vec![
                        "/y ",
                        &rainbowfy(message.strip_prefix(".rby ").unwrap().to_string()),
                    ]
                    .concat(),
                );
                return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
            } else if message.starts_with(".rbm ") {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_string(
                    vec![
                        "/me ",
                        &rainbowfy(message.strip_prefix(".rbm ").unwrap().to_string()),
                    ]
                    .concat(),
                );
                return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
            } else if message.starts_with(".o ") {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_string("en_us".to_string());
                raw_packet.encode_byte(15);
                raw_packet.encode_varint(0);
                raw_packet.encode_bool(true);

                if message.contains("off") {
                    raw_packet.encode_ubyte(65);
                } else {
                    raw_packet.encode_ubyte(127);
                }
                raw_packet.encode_varint(1);

                return_packet_vec.push((Packet::from(raw_packet, 0x05), Direction::Serverbound));
            } else {
                return_packet_vec.push((
                    generate_message_packet("Command not found!").unwrap(),
                    Direction::Clientbound,
                ));
            }
        } else {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_string(message.to_string());
            return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
        }

        Ok((return_packet_vec, status))
    }
}
