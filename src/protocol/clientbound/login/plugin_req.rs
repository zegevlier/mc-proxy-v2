use crate::{packet, utils};
use hex::encode;

packet! { PluginRequest, -disp,
    {
    message_id: i32,
    channel: String,
    data: Vec<u8>,
}
}

impl Parsable for PluginRequest {}

impl std::fmt::Display for PluginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.message_id,
            self.channel,
            utils::make_string_fixed_length(encode(&self.data), 30)
        )
    }
}
