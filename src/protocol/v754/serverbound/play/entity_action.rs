use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
};
use crate::{packet::Packet, parsable::Parsable, raw_packet::RawPacket};
use crate::{Direction, SharedState};
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
enum Action {
    StartSneaking,
    StopSneaking,
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartJumpWithHorse,
    StopJumpWithHorse,
    OpenHorseInventory,
    StartFlyingWithElytra,
}

#[derive(Clone, Serialize)]
pub struct EntityAction {
    entity_id: i32,
    action_id: Action,
    jump_boost: i32,
}

#[async_trait::async_trait]
impl Parsable for EntityAction {
    fn empty() -> Self {
        Self {
            entity_id: 0,
            action_id: Action::StartSneaking,
            jump_boost: 0,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.entity_id = packet.decode_varint()?;
        self.action_id = match packet.decode_varint()? {
            0 => Action::StartSneaking,
            1 => Action::StopSneaking,
            2 => Action::LeaveBed,
            3 => Action::StartSprinting,
            4 => Action::StopSprinting,
            5 => Action::StartJumpWithHorse,
            6 => Action::StopJumpWithHorse,
            7 => Action::OpenHorseInventory,
            8 => Action::StartFlyingWithElytra,
            _ => unreachable!(),
        };
        self.jump_boost = packet.decode_varint()?;

        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:?} {}",
            self.entity_id, self.action_id, self.jump_boost,
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

        let mut raw_packet = RawPacket::new();

        raw_packet.encode_varint(self.entity_id);
        raw_packet.encode_varint(match self.action_id {
            Action::StartSneaking => 0,
            Action::StopSneaking => 1,
            Action::LeaveBed => 2,
            Action::StartSprinting => 3,
            Action::StopSprinting => 4,
            Action::StartJumpWithHorse => 5,
            Action::StopJumpWithHorse => 6,
            Action::OpenHorseInventory => 7,
            Action::StartFlyingWithElytra => 8,
        });
        raw_packet.encode_varint(100 * 100);

        return_packet_vec.push((
            Packet::from(raw_packet, fid_to_pid(Fid::EntityAction)),
            Direction::Serverbound,
        ));

        Ok(return_packet_vec)
    }
}
