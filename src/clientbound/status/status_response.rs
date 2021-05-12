use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone)]
pub struct StatusResponse {
    json_response: String,
}

impl Parsable for StatusResponse {
    fn empty() -> Self {
        Self {
            json_response: "".into(),
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
