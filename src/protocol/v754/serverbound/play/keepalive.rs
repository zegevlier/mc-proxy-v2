use crate::{
    conf::Configuration, packet::Packet, parsable::Parsable, raw_packet::RawPacket, Direction,
    EventHandler, SharedState,
};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct KeepAliveSb {
    keep_alive_id: i64,
}

#[async_trait::async_trait]
impl Parsable for KeepAliveSb {
    fn default() -> Self {
        Self { keep_alive_id: 0 }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.keep_alive_id = packet.decode_long()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{}", self.keep_alive_id,)
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
            Packet::from(
                {
                    let mut raw_packet = RawPacket::new();
                    raw_packet.encode_ubyte(7);
                    raw_packet.encode_float(7f32);
                    raw_packet
                },
                0x1D,
            ),
            Direction::Clientbound,
        )]);
    }
}
