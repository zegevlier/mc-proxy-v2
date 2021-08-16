use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
enum DiggingStatus {
    Started,
    Cancelled,
    Finished,
}

#[derive(Clone, Serialize)]
pub struct AckPlayerDigging {
    x: i64,
    y: i64,
    z: i64,
    block: i32,
    status: DiggingStatus,
    successful: bool,
}

impl Parsable for AckPlayerDigging {
    fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            block: 0,
            status: DiggingStatus::Started,
            successful: false,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        let position = packet.decode_position()?;
        self.x = position.0;
        self.y = position.1;
        self.z = position.2;
        self.block = packet.decode_varint()?;
        self.status = match packet.decode_varint()? {
            0x00 => DiggingStatus::Started,
            0x01 => DiggingStatus::Cancelled,
            0x02 => DiggingStatus::Finished,
            _ => return Err(()),
        };
        self.successful = packet.decode_bool()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {:?} {}",
            self.x, self.y, self.z, self.block, self.status, self.successful,
        )
    }
}
