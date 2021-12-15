use crate::packet;
use packet::{SharedState, State};

packet! {
    StatusPong, all, {
        payload: i64,
    }
}

impl Parsable for StatusPong {
    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = State::Handshaking;
        log::debug!("State updated to Handshaking");
        Ok(())
    }
}
