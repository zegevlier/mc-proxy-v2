use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct StatusPing {
    payload: i64,
}

impl Parsable for StatusPing {
    fn default() -> Self {
        Self { payload: 0 }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.payload = packet.decode_long()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{}", self.payload)
    }
}
