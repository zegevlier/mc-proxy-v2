use crate::utils::generate_message_packet;
use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};

#[derive(Clone)]
pub struct UpdateHealth {
    health: f32,
    food: i32,
    food_saturation: f32,
}

#[async_trait::async_trait]
impl Parsable for UpdateHealth {
    fn empty() -> Self {
        Self {
            health: 0f32,
            food: 0,
            food_saturation: 0f32,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.health = packet.decode_float()?;
        self.food = packet.decode_varint()?;
        self.food_saturation = packet.decode_float()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{} {} {}", self.health, self.food, self.food_saturation)
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: SharedState,
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
        let mut return_packet_vec = Vec::new();
        let mut raw_packet = RawPacket::new();
        raw_packet.encode_float(self.health)?;
        raw_packet.encode_varint({
            match self.food {
                c if c > 6 => self.food,
                c if c == 6 => {
                    return_packet_vec.push((
                        generate_message_packet("Food manipulating starting!!!").unwrap(),
                        Direction::Clientbound,
                    ));
                    7
                }
                _ => 7,
            }
        })?;
        raw_packet.encode_float(self.food_saturation)?;
        return_packet_vec.push((Packet::from(raw_packet, 0x49), Direction::Clientbound));
        Ok((return_packet_vec, status))
    }
}
