use crate::{parsable::Parsable, raw_packet::RawPacket};
use hex::encode;

use crate::utils;

#[derive(Clone)]
pub struct LoginStart {
    username: String,
}

impl Parsable for LoginStart {
    fn empty() -> Self {
        Self {
            username: "".into(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.username = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        self.username.to_string()
    }
}

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

    // fn packet_editing(&self) -> bool {
    //     true
    // }

    // async fn edit_packet(
    //     &self,
    //     state: SharedState,
    // ) -> Result<(Packet, Direction, SharedState), ()> {
    //     let mut state = state;
    //     state.ps_cipher.enable(&decode(&state.secret_key).unwrap());
    //     state.sp_cipher.enable(&decode(&state.secret_key).unwrap());

    //     log::debug!("Updated cipher with secret key");
    //     Ok((Packet::new(), Direction::Serverbound, state))
    // }
}

#[derive(Clone)]
pub struct PluginResponse {
    message_id: i32,
    success: bool,
    data: Vec<u8>,
}

impl Parsable for PluginResponse {
    fn empty() -> Self {
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
