use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ResourcePackSend {
    url: String,
    hash: String,
}

impl Parsable for ResourcePackSend {
    fn empty() -> Self {
        Self {
            url: String::new(),
            hash: String::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.url = packet.decode_string()?;
        self.hash = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{} {}", self.url, self.hash,)
    }
}
