use crate::parsable::Parsable;
use hex::encode;

use packet::{RawPacket, SafeDefault, Varint};

use crate::utils;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct EncResponse {
    shared_secret_length: Varint,
    shared_secret: Vec<u8>,
    verify_token_length: Varint,
    verify_token: Vec<u8>,
}

#[async_trait::async_trait]
impl Parsable for EncResponse {}

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

impl SafeDefault for EncResponse {
    fn default() -> Self {
        Self {
            shared_secret_length: Default::default(),
            shared_secret: Vec::new(),
            verify_token_length: Default::default(),
            verify_token: Vec::new(),
        }
    }
}

impl packet::ProtoDec for EncResponse {
    fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
        self.shared_secret_length = p.decode()?;
        self.shared_secret = p.read(self.shared_secret_length.into())?;
        self.verify_token_length = p.decode()?;
        self.verify_token = p.read(self.verify_token_length.into())?;
        Ok(())
    }
}

impl packet::ProtoEnc for EncResponse {
    fn encode(&self, p: &mut RawPacket) -> packet::Result<()> {
        p.encode(&self.shared_secret_length)?;
        p.push_slice(&self.shared_secret);
        p.encode(&self.verify_token_length)?;
        p.push_slice(&self.verify_token);
        Ok(())
    }
}
