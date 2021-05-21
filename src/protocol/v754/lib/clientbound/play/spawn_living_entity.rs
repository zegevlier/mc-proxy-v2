use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone)]
pub struct SpawnLivingEntity {
    entity_id: i32,
    object_uuid: u128,
    r#type: i32,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
    head_pitch: u8,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

impl Parsable for SpawnLivingEntity {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            object_uuid: 0,
            r#type: 0,
            x: 0f64,
            y: 0f64,
            z: 0f64,
            yaw: 0,
            pitch: 0,
            head_pitch: 0,
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
        self.yaw = packet.read(1)?[0];
        self.pitch = packet.read(1)?[0];
        self.head_pitch = packet.read(1)?[0];
        self.velocity_x = packet.decode_short()?;
        self.velocity_y = packet.decode_short()?;
        self.velocity_z = packet.decode_short()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:x} {} {} {} {} {} {} {} {} {} {}",
            self.entity_id,
            self.object_uuid,
            self.r#type,
            self.x,
            self.y,
            self.z,
            self.yaw,
            self.pitch,
            self.head_pitch,
            self.velocity_x,
            self.velocity_y,
            self.velocity_z
        )
    }
}
