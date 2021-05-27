use crate::{functions, packet::Packet, plugin, raw_packet::RawPacket, Direction};

#[derive(Clone)]
pub struct WeirdSky {}

impl plugin::EventHandler for WeirdSky {
    fn new() -> Self {
        Self {}
    }

    fn on_message(
        &mut self,
        message: &functions::serverbound::play::ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        if message.message.starts_with(".sky ") {
            let sky_numer = match message.message.split(' ').nth(1) {
                Some(n) => n.parse(),
                None => return None,
            }
            .unwrap();
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_ubyte(7);
            raw_packet.encode_float(sky_numer);
            Some(vec![(
                Packet::from(raw_packet, 0x1D),
                Direction::Clientbound,
            )])
        } else {
            None
        }
    }
}
