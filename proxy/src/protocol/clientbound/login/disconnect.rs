use crate::{packet, SharedState, State};
use packet::Chat;

packet! {
    Disconnect, all,
    {
        reason: Chat,
    }
}

impl Parsable for Disconnect {
    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Handshaking;
        log::debug!("State updated to Handshaking");
        Ok(())
    }
}
