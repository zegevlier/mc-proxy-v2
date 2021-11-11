use crate::parsable::Parsable;
use hex::encode;

use packet::{RawPacket, VarInt};

use crate::utils;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct EncResponse {
    shared_secret_length: VarInt,
    shared_secret: Vec<u8>,
    verify_token_length: VarInt,
    verify_token: Vec<u8>,
}

#[async_trait::async_trait]
impl Parsable for EncResponse {
    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.shared_secret_length = packet.decode()?;
        self.shared_secret = packet.read(self.shared_secret_length.into())?;
        self.verify_token_length = packet.decode()?;
        self.verify_token = packet.read(self.verify_token_length.into())?;
        Ok(())
    }
}

impl std::fmt::Display for EncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.shared_secret_length,
            utils::make_string_fixed_length(encode(self.shared_secret.clone()), 20),
            self.verify_token_length,
            utils::make_string_fixed_length(encode(self.verify_token.clone()), 20)
        )
    }
}

impl crate::parsable::SafeDefault for EncResponse {
    fn default() -> Self {
        Self {
            shared_secret_length: Default::default(),
            shared_secret: Vec::new(),
            verify_token_length: Default::default(),
            verify_token: Vec::new(),
        }
    }
}
