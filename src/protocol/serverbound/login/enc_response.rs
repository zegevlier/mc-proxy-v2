use crate::{packet, utils};
use packet::LenPrefixedVec;

use hex::encode;

packet! {EncResponse, -disp,
    {
        shared_secret: LenPrefixedVec<u8>,
        verify_token: LenPrefixedVec<u8>,
    }
}

impl std::fmt::Display for EncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            utils::make_string_fixed_length(encode(self.shared_secret.v.clone()), 20),
            utils::make_string_fixed_length(encode(self.verify_token.v.clone()), 20)
        )
    }
}

impl Parsable for EncResponse {}
