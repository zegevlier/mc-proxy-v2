use crate::parsable::Parsable;
use serde::Serialize;

use packet::{RawPacket, SafeDefault};

#[derive(Clone, Serialize)]
pub struct StatusPing {
    payload: i64,
}

impl Parsable for StatusPing {}

impl std::fmt::Display for StatusPing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.payload)
    }
}

impl SafeDefault for StatusPing {
    fn default() -> Self {
        Self { payload: 0 }
    }
}
impl packet::ProtoDec for StatusPing {
    fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
        self.payload = p.decode()?;
        Ok(())
    }
}

impl packet::ProtoEnc for StatusPing {
    fn encode(&self, p: &mut RawPacket) {
        p.encode(&self.payload);
    }
}
