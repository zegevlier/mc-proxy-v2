use crate::parsable::Parsable;
use crate::SharedState;

use packet::{RawPacket, SafeDefault, VarInt};

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SetCompression {
    threshold: VarInt,
}

impl Parsable for SetCompression {
    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.compress = self.threshold.to::<i32>() as u32;
        Ok(())
    }
}

impl std::fmt::Display for SetCompression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.threshold)
    }
}

impl SafeDefault for SetCompression {
    fn default() -> Self {
        Self {
            threshold: Default::default(),
        }
    }
}

impl packet::ProtoDec for SetCompression {
    fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
        self.threshold = p.decode()?;
        Ok(())
    }
}

impl packet::ProtoEnc for SetCompression {
    fn encode(&self, p: &mut RawPacket) {
        p.encode(&self.threshold);
    }
}
