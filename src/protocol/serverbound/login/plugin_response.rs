use crate::{packet, utils};
use hex::encode;

packet! { PluginResponse, -disp, {
    message_id: i32,
    success: bool,
    data: Vec<u8>,
}}

impl Parsable for PluginResponse {}

impl std::fmt::Display for PluginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.message_id,
            self.success,
            utils::make_string_fixed_length(encode(&self.data), 30)
        )
    }
}
