use crate::{parsable::Parsable, raw_packet::RawPacket};
use crate::{SharedState, State};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Disconnect {
    reason: String,
}

impl Parsable for Disconnect {
    fn empty() -> Self {
        Self { reason: "".into() }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.reason = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        self.reason.to_string()
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
