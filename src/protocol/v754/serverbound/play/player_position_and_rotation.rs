use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone)]
pub struct PlayerPositionRotation {
    x: f64,
    feet_y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}

#[async_trait::async_trait]
impl Parsable for PlayerPositionRotation {
    fn empty() -> Self {
        Self {
            x: 0f64,
            feet_y: 0f64,
            z: 0f64,
            yaw: 0f32,
            pitch: 0f32,
            on_ground: false,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.x = packet.decode_double()?;
        self.feet_y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.yaw = packet.decode_float()?;
        self.pitch = packet.decode_float()?;
        self.on_ground = packet.decode_bool()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.x, self.feet_y, self.z, self.yaw, self.pitch, self.on_ground,
        )
    }
}
