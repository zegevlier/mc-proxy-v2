use crate::{parsable::Parsable, raw_packet::RawPacket};
use hex::encode;

use crate::utils;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PluginResponse {
    message_id: i32,
    success: bool,
    data: Vec<u8>,
}

impl Parsable for PluginResponse {
    fn default() -> Self {
        Self {
            message_id: 0,
            success: false,
            data: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.message_id = packet.decode_varint()?;
        self.success = packet.decode_bool()?;
        self.data = packet.get_vec();
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {}",
            self.message_id,
            self.success,
            utils::make_string_fixed_length(encode(&self.data), 30)
        )
    }
}
