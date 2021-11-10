use crate::functions::fid_to_pid;
use crate::packet::Packet;
use crate::{parsable::Parsable, raw_packet::RawPacket};
use crate::{SharedState, State};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: State,
}

impl Parsable for Handshake {
    fn default() -> Self {
        Handshake {
            protocol_version: 0,
            server_address: String::new(),
            server_port: 0,
            next_state: State::Handshaking,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.protocol_version = packet.decode_varint()?;
        self.server_address = packet.decode_string()?;
        self.server_port = packet.decode_ushort()?;
        self.next_state = match packet.decode_varint()? {
            1 => State::Status,
            2 => State::Login,
            _ => return Err(()),
        };
        Ok(())
    }

    fn encode_packet(&self) -> Result<Packet, ()> {
        let mut raw_packet = RawPacket::new();
        raw_packet.encode_varint(self.protocol_version);
        raw_packet.encode_string(self.server_address.to_owned());
        raw_packet.encode_ushort(self.server_port);
        raw_packet.encode_varint(match self.next_state {
            State::Status => 1,
            State::Login => 2,
            _ => return Err(()),
        });
        Ok(Packet::from(
            raw_packet,
            fid_to_pid(crate::functions::Fid::Handshake),
        ))
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {}:{} {:?}",
            self.protocol_version, self.server_address, self.server_port, self.next_state
        )
    }

    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.state = self.next_state;
        log::debug!("State updated to {:?}", status.state);
        Ok(())
    }
}
