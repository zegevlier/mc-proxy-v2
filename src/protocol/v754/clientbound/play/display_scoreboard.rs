use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone, Debug)]
enum ScoreboardPosition {
    List,
    Sidebar,
    BelowName,
    TeamSpecificSidebar,
}

#[derive(Clone)]
pub struct DisplayScoreboard {
    position: ScoreboardPosition,
    name: String,
}

impl Parsable for DisplayScoreboard {
    fn empty() -> Self {
        Self {
            name: String::new(),
            position: ScoreboardPosition::List,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.name = packet.decode_string()?;
        self.position = match packet.decode_byte()? {
            0 => ScoreboardPosition::List,
            1 => ScoreboardPosition::Sidebar,
            2 => ScoreboardPosition::BelowName,
            _ => ScoreboardPosition::TeamSpecificSidebar,
        };
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{:?} {}", self.position, self.name)
    }
}