use crate::{
    conf::Configuration, packet::Packet, parsable::Parsable, raw_packet::RawPacket, Direction,
    EventHandler, SharedState,
};

#[derive(Clone)]
pub struct KeepAliveSb {
    keep_alive_id: i64,
}

#[async_trait::async_trait]
impl Parsable for KeepAliveSb {
    fn empty() -> Self {
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
        status: SharedState,
        _: &mut Vec<Box<dyn EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
        return Ok((
            vec![(
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
            )],
            status,
        ));
    }
}
