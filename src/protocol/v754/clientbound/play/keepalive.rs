use crate::{
    functions::{fid_to_pid, Fid},
    packet::Packet,
    parsable::Parsable,
    raw_packet::RawPacket,
    Direction, EventHandler, SharedState,
};

#[derive(Clone)]
pub struct KeepAliveCb {
    keep_alive_id: i64,
}

#[async_trait::async_trait]
impl Parsable for KeepAliveCb {
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
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
        return Ok((
            vec![
                (
                    Packet::from(
                        {
                            let mut raw_packet = RawPacket::new();
                            raw_packet.encode_long(self.keep_alive_id);
                            raw_packet
                        },
                        fid_to_pid(Fid::KeepAliveSb),
                    ),
                    Direction::Serverbound,
                ),
                (
                    Packet::from(
                        {
                            let mut raw_packet = RawPacket::new();
                            raw_packet.encode_long(self.keep_alive_id);
                            raw_packet
                        },
                        fid_to_pid(Fid::KeepAliveCb),
                    ),
                    Direction::Clientbound,
                ),
            ],
            status,
        ));
    }
}
