use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
    utils::generate_message_packet,
};
use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct UpdateHealth {
    health: f32,
    food: i32,
    food_saturation: f32,
}

#[async_trait::async_trait]
impl Parsable for UpdateHealth {
    fn default() -> Self {
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
        _status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        let mut return_packet_vec = Vec::new();
        if self.food == 7 {
            return_packet_vec.push((
                generate_message_packet("Sent /eat command").unwrap(),
                Direction::Clientbound,
            ));
            let mut eat_command = RawPacket::new();
            eat_command.encode_string("/eat".to_string());
            return_packet_vec.push((
                Packet::from(eat_command, fid_to_pid(Fid::ChatMessageServerbound)),
                Direction::Serverbound,
            ));
        };

        Ok(return_packet_vec)
    }
}
