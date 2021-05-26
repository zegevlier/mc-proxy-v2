use crate::{
    functions::{clientbound::play::PlayerAbilities, fid_to_pid},
    packet::Packet,
    plugin,
    raw_packet::RawPacket,
    Direction,
};

#[derive(Clone)]
pub struct FakeFly {}

impl plugin::EventHandler for FakeFly {
    fn new() -> Self {
        Self {}
    }

    fn on_player_abilities(
        &mut self,
        player_abilities: &PlayerAbilities,
    ) -> Option<Vec<(Packet, Direction)>> {
        Some(vec![(
            {
                let mut new_packet = RawPacket::new();
                new_packet.encode_ubyte(player_abilities.flags | 0x04);
                new_packet.encode_float(player_abilities.flying_speed);
                new_packet.encode_float(player_abilities.fov_modifier);
                Packet::from(
                    new_packet,
                    fid_to_pid(crate::functions::Fid::PlayerAbilities),
                )
            },
            Direction::Clientbound,
        )])
    }
}
