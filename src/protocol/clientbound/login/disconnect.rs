use crate::parsable::Parsable;
use crate::{SharedState, State};
use packet::RawPacket;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Disconnect {
    reason: String,
}

impl Parsable for Disconnect {
    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.reason = packet.decode()?;
        Ok(())
    }

    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Handshaking;
        log::debug!("State updated to Handshaking");
        Ok(())
    }
}

impl std::fmt::Display for Disconnect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl crate::parsable::SafeDefault for Disconnect {
    fn default() -> Self {
        Self {
            reason: String::new(),
        }
    }
}
