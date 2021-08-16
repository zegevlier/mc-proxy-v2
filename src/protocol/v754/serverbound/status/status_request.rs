use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Serialize)]
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
