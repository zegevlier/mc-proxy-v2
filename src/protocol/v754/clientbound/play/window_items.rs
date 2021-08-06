use crate::{
    functions::{fid_to_pid, Fid},
    packet::Packet,
    parsable::Parsable,
    raw_packet::RawPacket,
    types::Slot,
    Direction,
};

#[derive(Clone)]
pub struct WindowItems {
    window_id: u8,
    count: i16,
    slot_data: Vec<Slot>,
}

#[async_trait::async_trait]
impl Parsable for WindowItems {
    fn empty() -> Self {
        Self {
            window_id: 0,
            count: 0,
            slot_data: Vec::new(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.window_id = packet.decode_ubyte()?;
        self.count = packet.decode_short()?;
        let mut slot_list = Vec::new();
        for _ in 0..self.count {
            slot_list.push(packet.decode_slot()?);
        }
        self.slot_data = slot_list;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{} {:?}", self.window_id, self.slot_data)
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: crate::SharedState,
        _plugins: &mut Vec<Box<dyn crate::EventHandler + Send>>,
        _config: &crate::conf::Configuration,
    ) -> Result<
        (
            Vec<(crate::packet::Packet, crate::Direction)>,
            crate::SharedState,
        ),
        (),
    > {
        let mut raw_packet = RawPacket::new();
        let new_slot_data = self.slot_data.clone();
        raw_packet.encode_ubyte(self.window_id);
        raw_packet.encode_short(self.count);
        for slot in new_slot_data.iter() {
            raw_packet.encode_slot(slot.to_owned());
        }
        Ok((
            vec![(
                Packet::from(raw_packet, fid_to_pid(Fid::WindowItems)),
                Direction::Clientbound,
            )],
            status,
        ))
    }
}
