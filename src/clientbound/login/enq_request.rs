use crate::utils;
use crate::{parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};
use hex::encode;
use rand::Rng;
use regex::Regex;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use std::collections::HashMap;
use std::iter;

use num_bigint_dig::BigUint;
use reqwest::Client;
use rsa::{PaddingScheme, PublicKey, RSAPublicKey};
use rsa_der::public_key_from_der;
use rustc_serialize::hex::ToHex;

#[derive(Clone)]
pub struct EncRequest {
    server_id: String,
    public_key_length: i32,
    public_key: Vec<u8>,
    verify_token_length: i32,
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
    fn empty() -> Self {
        Self {
            server_id: "".into(),
            public_key_length: 0,
            public_key: Vec::new(),
            verify_token_length: 0,
            verify_token: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.server_id = packet.decode_string()?;
        self.public_key_length = packet.decode_varint()?;
        self.public_key = packet.read(self.public_key_length as usize)?;
        self.verify_token_length = packet.decode_varint()?;
        self.verify_token = packet.read(self.verify_token_length as usize)?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {}",
            // self.server_id,
            self.public_key_length,
            utils::make_string_fixed_length(encode(self.public_key.clone()), 20),
            self.verify_token_length,
            utils::make_string_fixed_length(encode(self.verify_token.clone()), 20)
        )
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: SharedState,
    ) -> Result<(RawPacket, Direction, SharedState), ()> {
        let mut status = status;
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
        req_map.insert("accessToken", "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2NTY0M2EzYmU0NGY4NmVlODE4OWEwMDllMGNlYTNmYSIsInlnZ3QiOiI1ZDQ5MzgyZDU4Nzk0NDE1OTg2MDU3Y2E3Njk4MWFjYSIsInNwciI6ImY1NGM3NGRkMzM2MjQyMmM4MGY5ZGE3MWVjYTRhYWEzIiwiaXNzIjoiWWdnZHJhc2lsLUF1dGgiLCJleHAiOjE2MjA4NDgwMTIsImlhdCI6MTYyMDY3NTIxMn0.TVkYMXHA3kuvHB1AE7oRXd6ojUd-cKnWLWZOkaxs520");
        req_map.insert("selectedProfile", "f54c74dd3362422c80f9da71eca4aaa3");
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
            RSAPublicKey::new(BigUint::from_bytes_be(&n), BigUint::from_bytes_be(&e)).unwrap();
        let padding = PaddingScheme::new_pkcs1v15_encrypt();

        let mut unformatted_packet = crate::RawPacket::new();
        unformatted_packet.encode_varint(1)?;
        unformatted_packet.encode_varint(128)?;
        unformatted_packet.push_vec(
            public_key
                .encrypt(&mut rng, padding, &status.secret_key[..])
                .unwrap(),
        );
        unformatted_packet.encode_varint(128)?;
        let padding = PaddingScheme::new_pkcs1v15_encrypt();

        unformatted_packet.push_vec(
            public_key
                .encrypt(&mut rng, padding, &self.verify_token[..])
                .unwrap(),
        );
        let mut response_packet = RawPacket::new();
        response_packet.encode_varint(unformatted_packet.len() as i32)?;
        response_packet.push_vec(unformatted_packet.get_vec());
        // log::debug!("{:?}", response_packet.get_vec());
        log::debug!("Sending serverbound enc response");

        // Send to proxy_server packet:
        // Shared key length (varint)
        // Shared key encrypted with public key (byte array)
        // Verify token length (varint)
        // Verify token encrypted with public key (byte array)

        Ok((response_packet, Direction::Serverbound, status))
    }

    fn post_send_updating(&self) -> bool {
        true
    }

    fn post_send_update(&self, status: &mut SharedState) -> Result<(), ()> {
        status.ps_cipher.enable(&status.secret_key);
        status.sp_cipher.enable(&status.secret_key);
        log::debug!("Enabled ciphers");
        Ok(())
    }
}
