use crate::{functions, packet::Packet, plugin, raw_packet::RawPacket, Direction};

#[derive(Clone)]
pub struct CopyBook {}

impl plugin::EventHandler for CopyBook {
    fn new() -> Self {
        Self {}
    }

    fn on_message(
        &mut self,
        message: &functions::serverbound::play::ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        let mut return_vec = vec![];
        if message.message.starts_with(".click ") {
            for _ in 0..message.message.split(' ').nth(1).unwrap().parse().unwrap() {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_varint(0);
                return_vec.push((Packet::from(raw_packet, 0x2F), Direction::Serverbound));
            }

            Some(return_vec)
        } else {
            None
        }
    }
}
