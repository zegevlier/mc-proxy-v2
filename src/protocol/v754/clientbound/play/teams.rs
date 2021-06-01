use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone, Debug, Eq, PartialEq)]
enum TeamMode {
    Create,
    Remove,
    Update,
    AddEntities,
    RemoveEntities,
}

#[derive(Clone)]
pub struct Teams {
    team_name: String,
    mode: TeamMode,
    team_display_name: String,
    friendly_flags: i8,
    name_tag_visibility: String,
    collision_rule: String,
    team_color: i32,
    team_prefix: String,
    team_suffix: String,
    entity_count: i32,
    entities: Vec<String>,
}

impl Parsable for Teams {
    fn empty() -> Self {
        Self {
            team_name: String::new(),
            mode: TeamMode::Create,
            team_display_name: String::new(),
            friendly_flags: 0,
            name_tag_visibility: String::new(),
            collision_rule: String::new(),
            team_color: 0,
            team_prefix: String::new(),
            team_suffix: String::new(),
            entity_count: 0,
            entities: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.team_name = packet.decode_string()?;
        self.mode = match packet.decode_byte()? {
            0 => {
                self.team_display_name = packet.decode_string()?;
                self.friendly_flags = packet.decode_byte()?;
                self.name_tag_visibility = packet.decode_string()?;
                self.collision_rule = packet.decode_string()?;
                self.team_color = packet.decode_varint()?;
                self.team_prefix = packet.decode_string()?;
                self.team_suffix = packet.decode_string()?;
                self.entity_count = packet.decode_varint()?;
                for _ in 0..self.entity_count {
                    self.entities.push(packet.decode_string()?);
                }
                TeamMode::Create
            }
            1 => TeamMode::Remove,
            2 => {
                self.team_display_name = packet.decode_string()?;
                self.friendly_flags = packet.decode_byte()?;
                self.name_tag_visibility = packet.decode_string()?;
                self.collision_rule = packet.decode_string()?;
                self.team_color = packet.decode_varint()?;
                self.team_prefix = packet.decode_string()?;
                self.team_suffix = packet.decode_string()?;
                TeamMode::Update
            }
            3 => {
                self.entity_count = packet.decode_varint()?;
                for _ in 0..self.entity_count {
                    self.entities.push(packet.decode_string()?);
                }
                TeamMode::AddEntities
            }
            4 => {
                self.entity_count = packet.decode_varint()?;
                for _ in 0..self.entity_count {
                    self.entities.push(packet.decode_string()?);
                }
                TeamMode::RemoveEntities
            }
            _ => return Err(()),
        };
        Ok(())
    }

    fn get_printable(&self) -> String {
        if self.mode == TeamMode::Update {
            format!(
                "{} {} {}",
                self.team_name,
                // self.mode,
                // self.team_display_name,
                // self.friendly_flags,
                // self.name_tag_visibility,
                // self.collision_rule,
                // self.team_color,
                self.team_prefix,
                self.team_suffix,
                // self.entities,
            )
        } else {
            String::new()
        }
    }
}
