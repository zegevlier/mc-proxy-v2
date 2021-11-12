use crate::parsable::Parsable;
use serde::Serialize;

use packet::{RawPacket, SafeDefault};

#[derive(Clone, Serialize)]
pub struct StatusRequest {}

impl Parsable for StatusRequest {}

impl std::fmt::Display for StatusRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl SafeDefault for StatusRequest {
    fn default() -> Self {
        Self {}
    }
}

impl packet::ProtoDec for StatusRequest {
    fn decode(&mut self, _p: &mut RawPacket) -> packet::Result<()> {
        Ok(())
    }
}

impl packet::ProtoEnc for StatusRequest {
    fn encode(&self, _p: &mut RawPacket) -> packet::Result<()> {
        Ok(())
    }
}
