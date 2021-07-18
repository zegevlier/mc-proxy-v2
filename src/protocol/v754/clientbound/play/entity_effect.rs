use crate::{
    conf::Configuration, functions::fid_to_pid, packet::Packet, parsable::Parsable,
    raw_packet::RawPacket, Direction, SharedState,
};
#[derive(Clone)]
pub struct EntityEffect {
    pub entity_id: i32,
    pub effect_id: u8,
    pub amplifier: u8,
    pub duration: i32,
    pub flags: u8,
}

#[async_trait::async_trait]
impl Parsable for EntityEffect {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            effect_id: 0,
            amplifier: 0,
            duration: 0,
            flags: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.effect_id = packet.decode_ubyte()?;
        self.amplifier = packet.decode_ubyte()?;
        self.duration = packet.decode_varint()?;
        self.flags = packet.decode_ubyte()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.entity_id, self.effect_id, self.amplifier, self.duration, self.flags
        )
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
            match plugin.on_potion_effect_apply(self) {
                Some(plugin_vec) => {
                    return_vec = Some(plugin_vec);
                    break;
                }
                None => continue,
            }
        }
        if return_vec.is_none() {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_varint(self.entity_id);
            raw_packet.encode_ubyte(self.effect_id);
            raw_packet.encode_ubyte(self.amplifier);
            raw_packet.encode_varint(self.duration);
            raw_packet.encode_ubyte(self.flags);
            return_vec = Some(vec![(
                Packet::from(raw_packet, fid_to_pid(crate::functions::Fid::EntityEffect)),
                Direction::Clientbound,
            )]);
        }

        Ok((return_vec.unwrap(), status))
    }
}
