use crate::parsable::Parsable;
use crate::{SharedState, State};
use serde::Serialize;

use packet::{RawPacket, SafeDefault, Uuid};

#[derive(Clone, Serialize)]
pub struct LoginSuccess {
    uuid: Uuid,
    username: String,
}

impl Parsable for LoginSuccess {
    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Play;
        log::debug!("State updated to Play");
        Ok(())
    }
}

impl std::fmt::Display for LoginSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.uuid, self.username,)
    }
}

impl SafeDefault for LoginSuccess {
    fn default() -> Self {
        Self {
            uuid: Uuid::from(0),
            username: String::new(),
        }
    }
}

impl packet::ProtoDec for LoginSuccess {
    fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
        self.uuid = p.decode()?;
        self.username = p.decode()?;
        Ok(())
    }
}

impl packet::ProtoEnc for LoginSuccess {
    fn encode(&self, p: &mut RawPacket) {
        p.encode(&self.uuid);
        p.encode(&self.username);
    }
}
