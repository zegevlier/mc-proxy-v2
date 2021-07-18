use crate::{
    functions::{self, fid_to_pid},
    packet::Packet,
    plugin,
    raw_packet::RawPacket,
    utils::generate_message_packet,
    Direction,
};

#[derive(Clone)]
pub struct JumpBoostCommand {
    current_amplifier: u8,
}

impl plugin::EventHandler for JumpBoostCommand {
    fn new() -> Self {
        Self {
            current_amplifier: 0,
        }
    }

    fn on_message(
        &mut self,
        message: &functions::serverbound::play::ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        if message.message.starts_with(".jb ") {
            let new_amplifier = match message.message.split(' ').nth(1) {
                Some(n) => n.parse::<u8>(),
                None => return None,
            }
            .unwrap()
                - 1;
            self.current_amplifier = new_amplifier;
            return Some(vec![(
                generate_message_packet(&format!(
                    "Set jump boost to {}!",
                    self.current_amplifier + 1
                ))
                .unwrap(),
                Direction::Clientbound,
            )]);
        }
        None
    }

    fn on_potion_effect_apply(
        &mut self,
        effect_info: &functions::clientbound::play::EntityEffect,
    ) -> Option<Vec<(Packet, Direction)>> {
        if effect_info.amplifier != self.current_amplifier && effect_info.effect_id == 8 {
            if self.current_amplifier == 255 {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_varint(effect_info.entity_id);
                raw_packet.encode_ubyte(effect_info.effect_id);
                return Some(vec![(
                    Packet::from(raw_packet, 0x37),
                    Direction::Clientbound,
                )]);
            } else {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_varint(effect_info.entity_id);
                raw_packet.encode_ubyte(effect_info.effect_id);
                raw_packet.encode_ubyte(self.current_amplifier);
                raw_packet.encode_varint(effect_info.duration);
                raw_packet.encode_ubyte(effect_info.flags);

                return Some(vec![(
                    Packet::from(raw_packet, fid_to_pid(functions::Fid::EntityEffect)),
                    Direction::Clientbound,
                )]);
            }
        }
        None
    }
}
