use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct UpdateScore {
    entity_name: String,
    action: i8,
    objective_name: String,
    value: Option<i32>,
}

impl Parsable for UpdateScore {
    fn empty() -> Self {
        Self {
            entity_name: String::new(),
            action: 0,
            objective_name: String::new(),
            value: None,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.entity_name = packet.decode_string()?;
        self.action = packet.decode_byte()?;
        self.objective_name = packet.decode_string()?;
        if self.action != 1 {
            self.value = Some(packet.decode_varint()?);
        }
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {:?}",
            self.entity_name, self.action, self.objective_name, self.value
        )
    }
}
