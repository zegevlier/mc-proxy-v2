use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct StatusResponse {
    json_response: String,
}

impl Parsable for StatusResponse {
    fn default() -> Self {
        Self {
            json_response: String::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.json_response = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        self.json_response.to_string()
    }
}
