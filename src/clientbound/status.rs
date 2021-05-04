use crate::{packet::Packet, parsable::Parsable};
use crate::{SharedState, State};

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

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.json_response = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        self.json_response.to_string()
    }
}

#[derive(Clone)]
pub struct StatusPong {
    payload: i64,
}

impl Parsable for StatusPong {
    fn empty() -> Self {
        Self { payload: 0 }
    }

    fn parse_packet(&mut self, mut packet: Packet) -> Result<(), ()> {
        self.payload = packet.decode_long()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{}", self.payload)
    }

    fn status_updating(&self) -> bool {
        true
    }

    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Handshaking;
        log::debug!("State updated to {:?}", status.state);
        Ok(())
    }
}
