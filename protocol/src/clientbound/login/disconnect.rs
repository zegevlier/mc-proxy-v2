use packet::{self, SharedState, State, Chat};
use crate::packet;

packet! {
    Disconnect, all,
    {
        reason: Chat,
    }
}

impl Parsable for Disconnect {
    fn update_status(&self, status: &mut SharedState) -> packet::Result<()> {
        status.state = State::Handshaking;
        log::debug!("State updated to Handshaking");
        Ok(())
    }
}
