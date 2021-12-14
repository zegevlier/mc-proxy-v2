use crate::packet;
use mcore::types::{SharedState, State};

use packet::Uuid;

packet! {
    LoginSuccess, all,
    {
        uuid: Uuid,
        username: String,
    }
}

impl Parsable for LoginSuccess {
    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Play;
        log::debug!("State updated to Play");
        Ok(())
    }
}
