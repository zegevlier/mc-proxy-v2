use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SpawnXpOrb {
    entity_id: i32,
    x: f64,
    y: f64,
    z: f64,
    count: i16,
}

impl Parsable for SpawnXpOrb {
    fn default() -> Self {
        Self {
            entity_id: 0,
            x: 0f64,
            y: 0f64,
            z: 0f64,
            count: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.x = packet.decode_double()?;
        self.y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.count = packet.decode_short()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.entity_id, self.x, self.y, self.z, self.count
        )
    }
}
