use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone)]
pub struct StatusRequest {}

impl Parsable for StatusRequest {
    fn empty() -> Self {
        Self {}
    }

    fn parse_packet(&mut self, mut _packet: RawPacket) -> Result<(), ()> {
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("",)
    }
}
