use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct TabCompleteMatch {
    mat: String,
    has_tooltip: bool,
    tooltip: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct TabCompleteClientbound {
    id: i32,
    start: i32,
    length: i32,
    count: i32,
    matches: Vec<TabCompleteMatch>,
}

impl Parsable for TabCompleteClientbound {
    fn empty() -> Self {
        Self {
            id: 0,
            start: 0,
            length: 0,
            count: 0,
            matches: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.id = packet.decode_varint()?;
        self.start = packet.decode_varint()?;
        self.length = packet.decode_varint()?;
        self.count = packet.decode_varint()?;
        for _ in 0..self.count {
            let mat = packet.decode_string()?;
            let has_tooltip = packet.decode_bool()?;
            let tooltip = if has_tooltip {
                Some(packet.decode_string()?)
            } else {
                None
            };
            self.matches.push(TabCompleteMatch {
                mat,
                has_tooltip,
                tooltip,
            });
        }
        Ok(())
    }

    fn get_printable(&self) -> String {
        // TODO: Fix this printing so it doesn't spam the entire console.
        format!(
            "{} {} {} {} {:?}",
            self.id, self.start, self.length, self.count, self.matches
        )
    }
}
