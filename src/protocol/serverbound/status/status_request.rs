use crate::parsable::Parsable;
use serde::Serialize;

use packet::RawPacket;

#[derive(Clone, Serialize)]
pub struct StatusRequest {}

impl Parsable for StatusRequest {
    fn parse_packet(&mut self, mut _packet: RawPacket) -> Result<(), ()> {
        Ok(())
    }
}

impl std::fmt::Display for StatusRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl crate::parsable::SafeDefault for StatusRequest {
    fn default() -> Self {
        Self {}
    }
}
