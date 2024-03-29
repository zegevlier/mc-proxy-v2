use crate::types::Uuid;
use crate::{parsable::Parsable, raw_packet::RawPacket};
use crate::{SharedState, State};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct LoginSuccess {
    uuid: Uuid,
    username: String,
}

impl Parsable for LoginSuccess {
    fn default() -> Self {
        Self {
            uuid: Uuid::from(0),
            username: String::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.uuid = packet.decode_uuid()?;
        self.username = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{} {}", self.uuid, self.username,)
    }

    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Play;
        log::debug!("State updated to Play");
        Ok(())
    }
}
