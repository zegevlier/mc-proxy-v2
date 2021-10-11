use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
};
use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SteerVehicle {
    sideways: f32,
    forward: f32,
    flags: u8,
}

#[async_trait::async_trait]
impl Parsable for SteerVehicle {
    fn empty() -> Self {
        Self {
            sideways: 0.0,
            forward: 0.0,
            flags: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.sideways = packet.decode_float()?;
        self.forward = packet.decode_float()?;
        self.flags = packet.decode_ubyte()?;

        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{} {} {}", self.sideways, self.forward, self.flags,)
    }

    fn packet_editing(&self) -> bool {
        false
    }

    async fn edit_packet(
        &self,
        _status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        let mut return_packet_vec = Vec::new();

        let mut raw_packet = RawPacket::new();

        raw_packet.encode_float(self.sideways);
        raw_packet.encode_float(self.forward);
        raw_packet.encode_ubyte(self.flags);

        return_packet_vec.push((
            Packet::from(raw_packet, fid_to_pid(Fid::SteerVehicle)),
            Direction::Serverbound,
        ));

        Ok(return_packet_vec)
    }
}
