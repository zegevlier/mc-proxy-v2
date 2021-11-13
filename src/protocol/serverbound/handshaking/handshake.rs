use crate::{packet, SharedState, State};

use packet::Varint;

packet! {
    Handshake, -disp, {
        protocol_version: Varint,
        server_address: String,
        server_port: u16,
        next_state: State,
    }
}

impl Parsable for Handshake {
    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = self.next_state;
        log::debug!("State updated to {:?}", status.state);
        Ok(())
    }
}

impl std::fmt::Display for Handshake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}:{} {:?}",
            self.protocol_version, self.server_address, self.server_port, self.next_state
        )
    }
}
