use crate::functions::fid_to_pid;
use crate::parsable::Parsable;
use crate::{SharedState, State};
use serde::Serialize;

use packet::{varint, Packet, RawPacket, SafeDefault, VarInt};

#[derive(Clone, Serialize)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: State,
}

impl Parsable for Handshake {
    fn encode_packet(&self) -> Result<Packet, ()> {
        let mut raw_packet = RawPacket::new();
        raw_packet.encode(&self.protocol_version);
        raw_packet.encode(&self.server_address.to_owned());
        raw_packet.encode(&self.server_port);
        raw_packet.encode(&varint!(match self.next_state {
            State::Status => 1,
            State::Login => 2,
            _ => return Err(()),
        }));
        Ok(Packet::from(
            raw_packet,
            fid_to_pid(crate::functions::Fid::Handshake),
        ))
    }

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

impl SafeDefault for Handshake {
    fn default() -> Self {
        Handshake {
            protocol_version: Default::default(),
            server_address: String::new(),
            server_port: 0,
            next_state: State::Handshaking,
        }
    }
}

impl packet::ProtoDec for Handshake {
    fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
        self.protocol_version = p.decode()?;
        self.server_address = p.decode()?;
        self.server_port = p.decode()?;
        self.next_state = match p.decode::<VarInt>()?.to::<i32>() {
            1 => State::Status,
            2 => State::Login,
            _ => return Err(packet::Error::InvalidVarintEnum),
        };
        Ok(())
    }
}

impl packet::ProtoEnc for Handshake {
    fn encode(&self, p: &mut RawPacket) {
        p.encode(&self.protocol_version);
        p.encode(&self.server_address);
        p.encode(&self.server_port);
        p.encode(&varint!(match self.next_state {
            State::Status => 1,
            State::Login => 2,
            _ => return,
        }));
    }
}
