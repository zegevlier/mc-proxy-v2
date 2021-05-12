use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone)]
pub struct ChatMessageServerbound {
    message: String,
}

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
}
