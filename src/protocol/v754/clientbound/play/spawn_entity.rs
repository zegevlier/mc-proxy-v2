use crate::{parsable::Parsable, raw_packet::RawPacket, types::Uuid};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SpawnEntity {
    entity_id: i32,
    object_uuid: Uuid,
    r#type: i32,
    x: f64,
    y: f64,
    z: f64,
    pitch: u8,
    yaw: u8,
    data: i32,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

impl Parsable for SpawnEntity {
    fn default() -> Self {
        Self {
            entity_id: 0,
            object_uuid: Uuid::from(0),
            r#type: 0,
            x: 0f64,
            y: 0f64,
            z: 0f64,
            pitch: 0,
            yaw: 0,
            data: 0,
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.object_uuid = packet.decode_uuid()?;
        self.r#type = packet.decode_varint()?;
        self.x = packet.decode_double()?;
        self.y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.pitch = packet.read(1)?[0];
        self.yaw = packet.read(1)?[0];
        self.data = packet.decode_int()?;
        self.velocity_x = packet.decode_short()?;
        self.velocity_y = packet.decode_short()?;
        self.velocity_z = packet.decode_short()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {} {} {}",
            self.entity_id,
            self.object_uuid,
            self.r#type,
            self.x,
            self.y,
            self.z,
            self.pitch,
            self.yaw,
            self.data,
            self.velocity_x,
            self.velocity_y,
            self.velocity_z
        )
    }
}
