use crate::{packet, utils};
use hex::encode;

packet! { 
    PluginRequest, all, {
        message_id: i32,
        channel: String,
        data: Vec<u8>,
    } |this| {
        format!("{} {} {}",
            this.message_id,
            this.channel,
            utils::make_string_fixed_length(encode(&this.data), 30))
    }
}

impl Parsable for PluginRequest {}
