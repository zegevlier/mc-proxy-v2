use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum ResourcePackResponse {
    Success,
    Declined,
    Failed,
    Accepted,
}

#[derive(Clone, Serialize)]
pub struct ResourcePackStatus {
    result: ResourcePackResponse,
}

impl Parsable for ResourcePackStatus {
    fn empty() -> Self {
        Self {
            result: ResourcePackResponse::Success,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.result = match packet.decode_varint()? {
            0 => ResourcePackResponse::Success,
            1 => ResourcePackResponse::Declined,
            2 => ResourcePackResponse::Failed,
            3 => ResourcePackResponse::Accepted,
            _ => return Err(()),
        };
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{:?}", self.result)
    }
}
