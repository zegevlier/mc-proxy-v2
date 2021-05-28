use crate::{functions, packet::Packet, plugin, raw_packet::RawPacket, Direction};

#[derive(Clone)]
pub struct UpdateGame {}

impl plugin::EventHandler for UpdateGame {
    fn new() -> Self {
        Self {}
    }

    fn on_message(
        &mut self,
        message: &functions::serverbound::play::ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        if message.message.starts_with(".state ") {
            let reason = match message.message.split(' ').nth(1) {
                Some(n) => n.parse(),
                None => return None,
            }
            .unwrap();

            let value = match message.message.split(' ').nth(2) {
                Some(n) => n.parse(),
                None => return None,
            }
            .unwrap();
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_ubyte(reason);
            raw_packet.encode_float(value);
            Some(vec![(
                Packet::from(raw_packet, 0x1D),
                Direction::Clientbound,
            )])
        } else {
            None
        }
    }
}
