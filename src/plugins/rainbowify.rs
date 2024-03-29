use crate::{
    functions, packet::Packet, plugin, raw_packet::RawPacket, utils::rainbowfy, Direction,
};
use regex::{Captures, Regex};

#[derive(Clone)]
pub struct Rainbowify {}

impl plugin::EventHandler for Rainbowify {
    fn new() -> Self {
        Self {}
    }

    fn on_message(
        &mut self,
        message: &functions::serverbound::play::ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        let exp = Regex::new(r"\{([a-zA-Z0-9 _+=]*)\}").unwrap();
        let new_message = exp
            .replace_all(&message.message, |caps: &Captures| {
                log::debug!(
                    "Running on message for rainbowify: {}",
                    caps.get(1).unwrap().as_str().to_string()
                );
                rainbowfy(caps.get(1).unwrap().as_str().to_string())
            })
            .to_string();
        if new_message != message.message {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_string(new_message);
            Some(vec![(
                Packet::from(
                    raw_packet,
                    functions::fid_to_pid(functions::Fid::ChatMessageServerbound),
                ),
                Direction::Serverbound,
            )])
        } else {
            None
        }
    }
}
