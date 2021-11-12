use crate::packet;
use crate::{SharedState, State};

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

// impl std::fmt::Display for LoginSuccess {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} {}", self.uuid, self.username,)
//     }
// }
