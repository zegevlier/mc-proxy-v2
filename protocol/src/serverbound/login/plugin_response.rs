use crate::{packet, utils};
use hex::encode;

packet! { 
    PluginResponse, all, {
        message_id: i32,
        success: bool,
        data: Vec<u8>,
    } |this| {
        format!("{} {} {}",
            this.message_id,
            this.success,
            utils::make_string_fixed_length(encode(&this.data), 30))
    }
}

impl Parsable for PluginResponse {}
