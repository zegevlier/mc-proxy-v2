use crate::parsable::Parsable;
use crate::{SharedState, State};
use packet::{RawPacket, SafeDefault};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Disconnect {
    reason: String,
}

impl Parsable for Disconnect {
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

impl SafeDefault for Disconnect {
    fn default() -> Self {
        Self {
            reason: String::new(),
        }
    }
}

impl packet::ProtoDec for Disconnect {
    fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
        self.reason = p.decode()?;
        Ok(())
    }
}

impl packet::ProtoEnc for Disconnect {
    fn encode(&self, p: &mut RawPacket) -> packet::Result<()> {
        p.encode(&self.reason)?;
        Ok(())
    }
}
