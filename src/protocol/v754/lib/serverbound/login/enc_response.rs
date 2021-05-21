use crate::{parsable::Parsable, raw_packet::RawPacket};
use hex::encode;

use crate::utils;

#[derive(Clone)]
pub struct EncResponse {
    shared_secret_length: i32,
    shared_secret: Vec<u8>,
    verify_token_length: i32,
    verify_token: Vec<u8>,
}

#[async_trait::async_trait]
impl Parsable for EncResponse {
    fn empty() -> Self {
        Self {
            shared_secret_length: 0,
            shared_secret: Vec::new(),
            verify_token_length: 0,
            verify_token: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.shared_secret_length = packet.decode_varint()?;
        self.shared_secret = packet.read(self.shared_secret_length as usize)?;
        self.verify_token_length = packet.decode_varint()?;
        self.verify_token = packet.read(self.verify_token_length as usize)?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {}",
            self.shared_secret_length,
            utils::make_string_fixed_length(encode(self.shared_secret.clone()), 20),
            self.verify_token_length,
            utils::make_string_fixed_length(encode(self.verify_token.clone()), 20)
        )
    }
}
