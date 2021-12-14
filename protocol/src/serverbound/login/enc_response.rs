use crate::{packet, utils};
use packet::LenPrefixedVec;

use hex::encode;

packet! {
    EncResponse, all, {
        shared_secret: LenPrefixedVec<u8>,
        verify_token: LenPrefixedVec<u8>,
    } |this| {
        format!("{} {}",
            utils::make_string_fixed_length(encode(this.shared_secret.v.clone()), 20),
            utils::make_string_fixed_length(encode(this.verify_token.v.clone()), 20)
        )
    }
}

impl Parsable for EncResponse {}
