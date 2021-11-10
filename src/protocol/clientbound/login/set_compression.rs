use crate::SharedState;
use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SetCompression {
    threshold: i32,
}

impl Parsable for SetCompression {
    fn default() -> Self {
        Self { threshold: 0 }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.threshold = packet.decode_varint()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{}", self.threshold)
    }

    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.compress = self.threshold as u32;
        Ok(())
    }
}
