use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone, Debug)]
pub enum ResourcePackResponse {
    Success,
    Declined,
    Failed,
    Accepted,
}

#[derive(Clone)]
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
