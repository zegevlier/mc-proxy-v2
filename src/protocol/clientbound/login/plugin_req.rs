use crate::utils;
use crate::{parsable::Parsable, raw_packet::RawPacket};
use hex::encode;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PluginRequest {
    message_id: i32,
    channel: String,
    data: Vec<u8>,
}

impl Parsable for PluginRequest {
    fn default() -> Self {
        Self {
            message_id: 0,
            channel: String::new(),
            data: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.message_id = packet.decode_varint()?;
        self.channel = packet.decode_string()?;
        self.data = packet.get_vec();
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {}",
            self.message_id,
            self.channel,
            utils::make_string_fixed_length(encode(&self.data), 30)
        )
    }
}
