use crate::{parsable::Parsable, raw_packet::RawPacket};
use crate::{SharedState, State};

#[derive(Clone)]
pub struct LoginSuccess {
    uuid: u128,
    username: String,
}

impl Parsable for LoginSuccess {
    fn empty() -> Self {
        Self {
            uuid: 0,
            username: "".into(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.uuid = packet.decode_uuid()?;
        self.username = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{:x} {}", self.uuid, self.username,)
    }

    fn status_updating(&self) -> bool {
        true
    }

    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Play;
        log::debug!("State updated to {:?}", status.state);
        Ok(())
    }
}
