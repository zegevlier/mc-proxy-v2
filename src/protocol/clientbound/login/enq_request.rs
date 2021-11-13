use crate::parsable::Parsable;
use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
    utils, Ciphers,
};
use crate::{Direction, SharedState};
use hex::encode;
use rand::Rng;
use regex::Regex;

use packet::{Packet, RawPacket, SafeDefault, Varint};

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use std::collections::HashMap;
use std::iter;

use num_bigint_dig::BigUint;
use reqwest::Client;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use rsa_der::public_key_from_der;
use rustc_serialize::hex::ToHex;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct EncRequest {
    server_id: String,
    public_key_length: Varint,
    public_key: Vec<u8>,
    verify_token_length: Varint,
    verify_token: Vec<u8>,
}

const LEADING_ZERO_REGEX: &str = r#"^0+"#;

fn two_complement(bytes: &mut Vec<u8>) {
    let mut carry = true;
    for i in (0..bytes.len()).rev() {
        bytes[i] = !bytes[i];
        if carry {
            carry = bytes[i] == 0xff;
            bytes[i] += 1;
        }
    }
}

#[async_trait::async_trait]
impl Parsable for EncRequest {
    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        status.secret_key = rand::thread_rng().gen::<[u8; 16]>();

        let mut hash = Sha1::new();

        hash.input(self.server_id.as_bytes());
        hash.input(&status.secret_key);
        hash.input(&self.public_key);

        let mut hex: Vec<u8> = iter::repeat(0).take((hash.output_bits() + 7) / 8).collect();
        hash.result(&mut hex);

        let regex = Regex::new(LEADING_ZERO_REGEX).unwrap();

        let result_hash = if (hex[0] & 0x80) == 0x80 {
            two_complement(&mut hex);
            format!(
                "-{}",
                regex
                    .replace(hex.as_slice().to_hex().as_str(), "")
                    .to_string()
            )
        } else {
            regex
                .replace(hex.as_slice().to_hex().as_str(), "")
                .to_string()
        };

        let mut req_map = HashMap::new();
        req_map.insert("accessToken", &status.access_token);
        req_map.insert("selectedProfile", &status.uuid);
        req_map.insert("serverId", &result_hash);

        let client = Client::new();
        let response = client
            .post("https://sessionserver.mojang.com/session/minecraft/join")
            .json(&req_map)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

        // Send post request to https://sessionserver.mojang.com/session/minecraft/join
        // with the following data:
        /*
        {
          "accessToken": "<accessToken>",
          "selectedProfile": "<player's uuid without dashes>",
          "serverId": "<serverHash>"
        }
        */

        // Then get a 204 no content back
        let mut rng = rand::rngs::OsRng;
        let (n, e) = public_key_from_der(&self.public_key).unwrap();
        let public_key =
            RsaPublicKey::new(BigUint::from_bytes_be(&n), BigUint::from_bytes_be(&e)).unwrap();
        let padding = PaddingScheme::new_pkcs1v15_encrypt();

        let mut unformatted_packet = crate::RawPacket::new();
        unformatted_packet.encode(&packet::varint!(128))?;
        unformatted_packet.push_vec(
            public_key
                .encrypt(&mut rng, padding, &status.secret_key[..])
                .unwrap(),
        );
        unformatted_packet.encode(&packet::varint!(128))?;
        let padding = PaddingScheme::new_pkcs1v15_encrypt();

        unformatted_packet.push_vec(
            public_key
                .encrypt(&mut rng, padding, &self.verify_token[..])
                .unwrap(),
        );
        let response_packet = Packet::from(unformatted_packet, fid_to_pid(Fid::EncResponse));
        log::debug!("Sending serverbound encryption response");

        // Send to proxy_server packet:
        // Shared key length (varint)
        // Shared key encrypted with public key (byte array)
        // Verify token length (varint)
        // Verify token encrypted with public key (byte array)

        // Reset the access_token to not keep it in memory needlessly.
        status.access_token = String::new();

        Ok(vec![(response_packet, Direction::Serverbound)])
    }

    fn post_send_update(&self, ciphers: &mut Ciphers, status: &SharedState) -> Result<(), ()> {
        ciphers.ps_cipher.enable(&status.secret_key);
        ciphers.sp_cipher.enable(&status.secret_key);
        log::debug!("Enabled ciphers");
        Ok(())
    }
}

impl std::fmt::Display for EncRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            // self.server_id,
            self.public_key_length,
            utils::make_string_fixed_length(encode(self.public_key.clone()), 20),
            self.verify_token_length,
            utils::make_string_fixed_length(encode(self.verify_token.clone()), 20)
        )
    }
}

impl SafeDefault for EncRequest {
    fn default() -> Self {
        Self {
            server_id: String::new(),
            public_key_length: Default::default(),
            public_key: Vec::new(),
            verify_token_length: Default::default(),
            verify_token: Vec::new(),
        }
    }
}

impl packet::ProtoDec for EncRequest {
    fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
        self.server_id = p.decode()?;
        self.public_key_length = p.decode()?;
        self.public_key = p.read(self.public_key_length.into())?;
        self.verify_token_length = p.decode()?;
        self.verify_token = p.read(self.verify_token_length.into())?;
        Ok(())
    }
}

impl packet::ProtoEnc for EncRequest {
    fn encode(&self, p: &mut RawPacket) -> packet::Result<()> {
        p.encode(&self.server_id)?;
        p.encode(&self.public_key_length)?;
        p.push_slice(&self.public_key);
        p.encode(&self.verify_token_length)?;
        p.push_slice(&self.verify_token);
        Ok(())
    }
}
