use crate::{functions, packet::Packet, plugin, raw_packet::RawPacket, Direction};

#[derive(Clone)]
pub struct JumpTrain {
    x: f64,
    y: f64,
    z: f64,
}

impl plugin::EventHandler for JumpTrain {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn on_message(
        &mut self,
        message: &functions::serverbound::play::ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        let mut return_vec = vec![];
        if message.message.starts_with(".jump ") {
            for _ in 0..message.message.split(' ').nth(1).unwrap().parse().unwrap() {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_double(self.x);
                raw_packet.encode_double(self.y + 0.41999998688698);
                raw_packet.encode_double(self.z);
                raw_packet.encode_bool(false);
                return_vec.push((Packet::from(raw_packet, 0x12), Direction::Serverbound));
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_double(self.x);
                raw_packet.encode_double(self.y);
                raw_packet.encode_double(self.z);
                raw_packet.encode_bool(true);
                return_vec.push((Packet::from(raw_packet, 0x12), Direction::Serverbound));
            }
            log::info!("Jumped at {} {} {}", self.x, self.y, self.z);

            Some(return_vec)
        } else {
            None
        }
    }

    fn on_move(&mut self, x: f64, y: f64, z: f64) -> Option<Vec<(Packet, Direction)>> {
        let error_margin = 0.0000001;
        if (self.x - x).abs() < error_margin
            && (self.y - y).abs() < error_margin
            && (self.z - z).abs() < error_margin
        {
            let mut return_vec = vec![];
            for _ in 0..35 {
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_double(self.x);
                raw_packet.encode_double(self.y + 0.41999998688698);
                raw_packet.encode_double(self.z);
                raw_packet.encode_bool(false);
                return_vec.push((Packet::from(raw_packet, 0x12), Direction::Serverbound));
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_double(self.x);
                raw_packet.encode_double(self.y);
                raw_packet.encode_double(self.z);
                raw_packet.encode_bool(true);
                return_vec.push((Packet::from(raw_packet, 0x12), Direction::Serverbound));
            }
            log::info!("Jumped at {} {} {}", self.x, self.y, self.z);
            return Some(return_vec);
        }

        self.x = x;
        self.y = y;
        self.z = z;
        None
    }
}
