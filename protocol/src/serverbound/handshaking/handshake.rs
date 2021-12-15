use crate::packet;
use packet::{SharedState, State};

use packet::Varint;

packet! {
    Handshake, all, {
        protocol_version: Varint,
        server_address: String,
        server_port: u16,
        next_state: State,
    } |this| {
        format!("{} {}:{} {:?}",
            this.protocol_version, 
            this.server_address, 
            this.server_port, 
            this.next_state)
    }
}

impl Parsable for Handshake {
    fn update_status(&self, status: &mut SharedState) -> packet::Result<()> {
        status.state = self.next_state;
        log::debug!("State updated to {:?}", status.state);
        Ok(())
    }
}
