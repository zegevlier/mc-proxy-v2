use crate::{functions, packet::Packet, plugin, raw_packet::RawPacket, Direction};

#[derive(Clone)]
pub struct Gamemode {}

impl plugin::EventHandler for Gamemode {
    fn new() -> Self {
        Self {}
    }

    fn on_message(
        &mut self,
        message: &functions::serverbound::play::ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        if message.message.starts_with(".gm") {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_ubyte(3);
            if message.message == ".gmc" {
                raw_packet.encode_float(1f32);
            } else if message.message == ".gms" {
                raw_packet.encode_float(0f32);
            } else if message.message == ".gma" {
                raw_packet.encode_float(2f32);
            } else if message.message == ".gmsp" {
                raw_packet.encode_float(3f32);
            } else {
                return None;
            }
            Some(vec![(
                Packet::from(raw_packet, 0x1D),
                Direction::Clientbound,
            )])
        } else {
            None
        }
    }
}
