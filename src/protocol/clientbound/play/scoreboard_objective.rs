use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
enum ScoreboardType {
    Integer,
    Hearts,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
enum ScoreboardMode {
    Create,
    Remove,
    UpdateDisplayText,
}

#[derive(Clone, Serialize)]
pub struct ScoreboardObjective {
    objective_name: String,
    mode: ScoreboardMode,
    objective_value: Option<String>,
    sb_type: Option<ScoreboardType>,
}

impl Parsable for ScoreboardObjective {
    fn default() -> Self {
        Self {
            objective_name: String::new(),
            mode: ScoreboardMode::Create,
            objective_value: None,
            sb_type: None,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.objective_name = packet.decode_string()?;
        self.mode = match packet.decode_byte()? {
            0 => ScoreboardMode::Create,
            1 => ScoreboardMode::Remove,
            2 => ScoreboardMode::UpdateDisplayText,
            _ => return Err(()),
        };
        if self.mode == ScoreboardMode::Create || self.mode == ScoreboardMode::UpdateDisplayText {
            self.objective_value = Some(packet.decode_string()?);
            self.sb_type = Some(match packet.decode_varint()? {
                0 => ScoreboardType::Integer,
                1 => ScoreboardType::Hearts,
                _ => return Err(()),
            });
        }
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:?} {:?} {:?}",
            self.objective_name, self.mode, self.objective_value, self.sb_type
        )
    }
}
