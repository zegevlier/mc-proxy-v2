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
    ) -> Result<(Packet, Direction, SharedState), ()> {
        let message = self.message.clone();
        let mut raw_packet = RawPacket::new();
        raw_packet.encode_string(message)?;
        Ok((
            Packet::from(raw_packet, 0x03),
            Direction::Serverbound,
            status,
        ))
    }
}
