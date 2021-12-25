use crate::{packet, utils};
use config_loader::Configuration;
use mcore::types::Direction;
use packet::{LenPrefixedVec, ProtoEnc, SharedState};

use std::collections::HashMap;

use hex::encode;
use rand::Rng;

use sha1::{Digest, Sha1};

use hyper::{Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use num_bigint_dig::BigUint;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use rsa_der::public_key_from_der;
use rustc_serialize::hex::ToHex;

packet! {
    EncRequest, all, {
        server_id: String,
        public_key: LenPrefixedVec<u8>,
        verify_token: LenPrefixedVec<u8>,
    } |this| {
        format!("{} {} {}",
        this.server_id,
        utils::make_string_fixed_length(encode(this.public_key.v.clone()), 20),
        utils::make_string_fixed_length(encode(this.verify_token.v.clone()), 20))
    }
}

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
        _plugins: &mut Vec<Box<dyn plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> packet::Result<Option<Vec<(Packet, Direction)>>> {
        status.secret_key = rand::thread_rng().gen::<[u8; 16]>();

        let mut hasher = Sha1::new();

        hasher.update(self.server_id.as_bytes());
        hasher.update(&status.secret_key);
        hasher.update(&self.public_key.v);

        let mut hex: Vec<u8> = hasher.finalize().to_vec();

        let result_hash = if (hex[0] & 0x80) == 0x80 {
            two_complement(&mut hex);
            format!("-{}", {
                let thing = hex.as_slice().to_hex();
                if thing.starts_with('0') {
                    thing.strip_prefix('0').unwrap().to_string()
                } else {
                    thing
                }
            })
        } else {
            let thing = hex.as_slice().to_hex();
            if thing.starts_with('0') {
                thing.strip_prefix('0').unwrap().to_string()
            } else {
                thing.to_string()
            }
        };

        let mut req_map = HashMap::new();
        req_map.insert("accessToken", &status.access_token);
        req_map.insert("selectedProfile", &status.uuid);
        req_map.insert("serverId", &result_hash);

        let https = HttpsConnector::new();

        let client = Client::builder().build::<_, hyper::Body>(https);

        let req = Request::builder()
            .method(Method::POST)
            .uri("https://sessionserver.mojang.com/session/minecraft/join")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&req_map).unwrap().into())
            .unwrap();

        let res = client.request(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        // Then get a 204 no content back
        let mut rng = rand::rngs::OsRng;
        let (n, e) = public_key_from_der(&self.public_key.v).unwrap();
        let public_key =
            RsaPublicKey::new(BigUint::from_bytes_be(&n), BigUint::from_bytes_be(&e)).unwrap();

        let response_packet = crate::serverbound::login::EncResponse {
            shared_secret: LenPrefixedVec::from(
                public_key
                    .encrypt(
                        &mut rng,
                        PaddingScheme::new_pkcs1v15_encrypt(),
                        &status.secret_key[..],
                    )
                    .unwrap(),
            ),
            verify_token: LenPrefixedVec::from(
                public_key
                    .encrypt(
                        &mut rng,
                        PaddingScheme::new_pkcs1v15_encrypt(),
                        &self.verify_token.v[..],
                    )
                    .unwrap(),
            ),
        }
        .encode_packet()?;
        log::debug!("Sending serverbound encryption response");

        // Reset the access_token to not keep it in memory needlessly.
        status.access_token = String::new();

        Ok(Some(vec![(response_packet, Direction::Serverbound)]))
    }

    fn post_send_update(
        &self,
        ciphers: &mut cipher::Ciphers,
        status: &SharedState,
    ) -> packet::Result<()> {
        ciphers.ps_cipher.enable(&status.secret_key);
        ciphers.sp_cipher.enable(&status.secret_key);
        log::debug!("Enabled ciphers");
        Ok(())
    }
}

// impl std::fmt::Display for EncRequest {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,

//         )
//     }
// }
