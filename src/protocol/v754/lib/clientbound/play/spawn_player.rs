use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone)]
pub struct SpawnPlayer {
    entity_id: i32,
    player_uuid: u128,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
}

impl Parsable for SpawnPlayer {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            player_uuid: 0,
            x: 0f64,
            y: 0f64,
            z: 0f64,
            yaw: 0,
            pitch: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.player_uuid = packet.decode_uuid()?;
        self.x = packet.decode_double()?;
        self.y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.yaw = packet.read(1)?[0];
        self.pitch = packet.read(1)?[0];
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:x} {} {} {} {} {}",
            self.entity_id, self.player_uuid, self.x, self.y, self.z, self.yaw, self.pitch,
        )
    }
}
