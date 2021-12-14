use packet::{Chat, Packet, ProtoEnc};
use regex::{Captures, Regex};

use protocol::serverbound::play::ChatMessageServerbound;
use mcore::types::Direction;

#[derive(Clone)]
pub struct Rainbowify {}

impl plugin::EventHandler for Rainbowify {
    fn new() -> Self {
        Self {}
    }

    fn on_message(&mut self, message: &Chat) -> Option<Vec<(Packet, Direction)>> {
        let exp = Regex::new(r"\{([a-zA-Z0-9 _+=]*)\}").unwrap();
        let new_message = exp
            .replace_all(message.get_string(), |caps: &Captures| {
                log::debug!(
                    "Running on message for rainbowify: {}",
                    caps.get(1).unwrap().as_str().to_string()
                );
                rainbowfy(caps.get(1).unwrap().as_str().to_string())
            })
            .to_string();
        if &new_message != message.get_string() {
            Some(vec![(
                ChatMessageServerbound {
                    message: Chat::from(new_message),
                }
                .encode_packet()
                .unwrap(),
                Direction::Serverbound,
            )])
        } else {
            None
        }
    }
}

pub fn rainbowfy(message: String) -> String {
    let mut return_message = String::new();
    let rainbow_characters = "c6eab5";
    for (i, cha) in message.chars().enumerate() {
        match cha {
            ' ' => return_message.push(cha),
            _ => {
                return_message.push('&');
                return_message.push(
                    rainbow_characters
                        .chars()
                        .nth(i % rainbow_characters.len())
                        .unwrap(),
                );
                return_message.push(cha);
            }
        }
    }
    return_message
}