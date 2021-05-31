use crate::{
    conf::Configuration, functions::fid_to_pid, packet::Packet, parsable::Parsable,
    raw_packet::RawPacket, Direction, SharedState,
};
#[derive(Clone)]
pub struct PlayerAbilities {
    pub flags: u8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

#[async_trait::async_trait]
impl Parsable for PlayerAbilities {
    fn empty() -> Self {
        Self {
            flags: 0,
            flying_speed: 0f32,
            fov_modifier: 0f32,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.flags = packet.decode_ubyte()?;
        self.flying_speed = packet.decode_float()?;
        self.fov_modifier = packet.decode_float()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{} {} {}", self.flags, self.flying_speed, self.fov_modifier)
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: SharedState,
        plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
        let mut return_vec = None;
        for plugin in plugins {
            match plugin.on_player_abilities(self) {
                Some(plugin_vec) => {
                    return_vec = Some(plugin_vec);
                    break;
                }
                None => continue,
            }
        }
        if return_vec.is_none() {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_ubyte(self.flags);
            raw_packet.encode_float(self.flying_speed);
            raw_packet.encode_float(self.fov_modifier);
            return_vec = Some(vec![(
                Packet::from(
                    raw_packet,
                    fid_to_pid(crate::functions::Fid::PlayerAbilities),
                ),
                Direction::Clientbound,
            )]);
        }

        Ok((return_vec.unwrap(), status))
    }
}
