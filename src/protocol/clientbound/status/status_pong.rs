use crate::{parsable::Parsable, raw_packet::RawPacket};
use crate::{SharedState, State};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct StatusPong {
    payload: i64,
}

impl Parsable for StatusPong {
    fn default() -> Self {
        Self { payload: 0 }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.payload = packet.decode_long()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{}", self.payload)
    }

    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Handshaking;
        log::debug!("State updated to Handshaking");
        Ok(())
    }
}
