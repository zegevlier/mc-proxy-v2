use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
};
use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PlayerPositionAndLook {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    flags: i8,
    teleport_id: i32,
    dismount_vehicle: bool,
}

#[async_trait::async_trait]
impl Parsable for PlayerPositionAndLook {
    fn empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
            teleport_id: 0,
            dismount_vehicle: false,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.x = packet.decode_double()?;
        self.y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.yaw = packet.decode_float()?;
        self.pitch = packet.decode_float()?;
        self.flags = packet.decode_byte()?;
        self.teleport_id = packet.decode_varint()?;
        self.dismount_vehicle = packet.decode_bool()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {}",
            self.x,
            self.y,
            self.z,
            self.yaw,
            self.pitch,
            self.flags,
            self.teleport_id,
            self.dismount_vehicle
        )
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        _status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        let mut return_packet_vec = Vec::new();

        let mut raw_packet = RawPacket::new();
        raw_packet.encode_double(self.x);
        raw_packet.encode_double(self.y);
        raw_packet.encode_double(self.z);
        raw_packet.encode_float(self.yaw);
        raw_packet.encode_float(self.pitch);
        raw_packet.encode_byte(self.flags);
        raw_packet.encode_varint(self.teleport_id);
        raw_packet.encode_bool(false);

        return_packet_vec.push((
            Packet::from(raw_packet, fid_to_pid(Fid::PlayerPositionAndLook)),
            Direction::Serverbound,
        ));

        Ok(return_packet_vec)
    }
}
