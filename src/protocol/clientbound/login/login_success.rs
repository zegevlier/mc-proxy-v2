use crate::parsable::Parsable;
use crate::{SharedState, State};
use serde::Serialize;

use packet::{RawPacket, Uuid};

#[derive(Clone, Serialize)]
pub struct LoginSuccess {
    uuid: Uuid,
    username: String,
}

impl Parsable for LoginSuccess {
    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.uuid = packet.decode()?;
        self.username = packet.decode()?;
        Ok(())
    }

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

impl crate::parsable::SafeDefault for LoginSuccess {
    fn default() -> Self {
        Self {
            uuid: Uuid::from(0),
            username: String::new(),
        }
    }
}
