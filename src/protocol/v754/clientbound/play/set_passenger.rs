use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
    utils::generate_message_packet,
};
use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SetPassenger {
    entity_id: i32,
    passenger_count: i32,
    passengers: Vec<i32>,
}

#[async_trait::async_trait]
impl Parsable for SetPassenger {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            passenger_count: 0,
            passengers: vec![],
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.passenger_count = packet.decode_varint()?;
        for _ in 0..self.passenger_count {
            self.passengers.push(packet.decode_varint()?);
        }
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {:?}",
            self.entity_id, self.passenger_count, self.passengers,
        )
    }

    fn packet_editing(&self) -> bool {
        false
    }

    async fn edit_packet(
        &self,
        _status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        let mut return_packet_vec = Vec::new();

        if self.passenger_count != 0 {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_varint(self.entity_id);
            raw_packet.encode_varint(self.passenger_count as i32);
            for p in self.passengers.clone() {
                raw_packet.encode_varint(p);
            }
            return_packet_vec.push((
                Packet::from(raw_packet, fid_to_pid(Fid::SetPassenger)),
                Direction::Clientbound,
            ));
        } else {
            return_packet_vec.push((
                generate_message_packet("Horse tried to kick you off!").unwrap(),
                Direction::Clientbound,
            ))
        }

        Ok(return_packet_vec)
    }
}
