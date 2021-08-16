use crate::{parsable::Parsable, raw_packet::RawPacket, types::Uuid};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
enum FacingDirection {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Serialize)]
pub struct SpawnPainting {
    entity_id: i32,
    object_uuid: Uuid,
    motive: i32,
    x: i64,
    y: i64,
    z: i64,
    direction: FacingDirection,
}

impl Parsable for SpawnPainting {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            object_uuid: Uuid::from(0),
            motive: 0,
            x: 0,
            y: 0,
            z: 0,
            direction: FacingDirection::North,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.object_uuid = packet.decode_uuid()?;
        self.motive = packet.decode_varint()?;
        let position = packet.decode_position()?;
        self.x = position.0;
        self.y = position.1;
        self.z = position.2;
        match packet.read(1)?[0] {
            0x00 => self.direction = FacingDirection::South,
            0x01 => self.direction = FacingDirection::West,
            0x02 => self.direction = FacingDirection::North,
            0x03 => self.direction = FacingDirection::East,
            _ => return Err(()),
        }
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {} {} {:?}",
            self.entity_id, self.object_uuid, self.motive, self.x, self.y, self.z, self.direction,
        )
    }
}
