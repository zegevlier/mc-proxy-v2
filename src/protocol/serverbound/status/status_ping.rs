use crate::parsable::Parsable;
use serde::Serialize;

use packet::RawPacket;

#[derive(Clone, Serialize)]
pub struct StatusPing {
    payload: i64,
}

impl Parsable for StatusPing {
    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.payload = packet.decode()?;
        Ok(())
    }
}

impl std::fmt::Display for StatusPing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.payload)
    }
}

impl crate::parsable::SafeDefault for StatusPing {
    fn default() -> Self {
        Self { payload: 0 }
    }
}
