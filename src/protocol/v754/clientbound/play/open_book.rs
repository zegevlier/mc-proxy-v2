use crate::{
    conf::Configuration, packet::Packet, parsable::Parsable, raw_packet::RawPacket,
    utils::generate_message_packet, Direction, EventHandler, SharedState,
};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct OpenBook {
    hand: i32,
}

#[async_trait::async_trait]
impl Parsable for OpenBook {
    fn empty() -> Self {
        Self { hand: 0 }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.hand = packet.decode_varint()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{}", self.hand,)
    }

    fn packet_editing(&self) -> bool {
        false
    }

    async fn edit_packet(
        &self,
        _status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        return Ok(vec![(
            generate_message_packet("Not opening book!").unwrap(),
            Direction::Clientbound,
        )]);
    }
}
