use crate::parsable::Parsable;
use crate::SharedState;

use packet::{RawPacket, VarInt};

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SetCompression {
    threshold: VarInt,
}

impl Parsable for SetCompression {
    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.threshold = packet.decode()?;
        Ok(())
    }

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

impl crate::parsable::SafeDefault for SetCompression {
    fn default() -> Self {
        Self {
            threshold: Default::default(),
        }
    }
}
