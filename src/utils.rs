use crate::{packet::Packet, raw_packet::RawPacket};

// This converts a long string into one that's shortened.
// alongstringlikethis would become alongs...kethis
pub fn make_string_fixed_length(string: String, length: usize) -> String {
    if string.len() <= length {
        string
    } else {
        let part_size = length - 3 / 2;
        format!(
            "{}...{}",
            string[0..part_size].to_string(),
            string[string.len() - part_size..].to_string()
        )
    }
}

pub fn generate_message_packet(text: &str) -> Result<Packet, ()> {
    let mut raw_packet = RawPacket::new();
    raw_packet.encode_string(format!(
        "{{\"extra\":[{{\"color\":\"red\",\"text\":\"proxy\"}},{{\"text\":\"> {}\"}}],\"text\":\"\"}}",
        text
    ));
    raw_packet.encode_byte(1);
    raw_packet.encode_uuid(0);
    Ok(Packet::from(raw_packet, 0x0E))
}
