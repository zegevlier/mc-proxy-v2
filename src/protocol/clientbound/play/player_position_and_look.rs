use crate::{parsable::Parsable, raw_packet::RawPacket};
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
    fn default() -> Self {
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
}
