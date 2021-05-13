use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};

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
        let message = self.message.clone();
        let mut raw_packet = RawPacket::new();
        raw_packet.encode_string(message)?;

        if self.message.contains("test") {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_string("{\"extra\":[{\"text\":\"<\"},{\"color\":\"red\",\"text\":\"proxy\"},{\"text\":\"> test recieved!\"}],\"text\":\"\"}".into())?;
            raw_packet.encode_byte(1)?;
            raw_packet.encode_uuid(0)?;
            return_packet_vec.push((Packet::from(raw_packet, 0x0E), Direction::Clientbound));
        } else {
            return_packet_vec.push((Packet::from(raw_packet, 0x03), Direction::Serverbound));
        }

        Ok((return_packet_vec, status))
    }
}
//
