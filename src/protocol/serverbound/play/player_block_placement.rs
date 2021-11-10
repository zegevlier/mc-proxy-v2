use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
    packet::Packet,
    parsable::Parsable,
    raw_packet::RawPacket,
    Direction, EventHandler, SharedState,
};
use serde::Serialize;

/*
Hand	VarInt Enum	The hand from which the block is placed; 0: main hand, 1: off hand.
Location	Position	Block position.
Face	VarInt Enum	The face on which the block is placed (as documented at Player Digging).
Cursor Position X	Float	The position of the crosshair on the block, from 0 to 1 increasing from west to east.
Cursor Position Y	Float	The position of the crosshair on the block, from 0 to 1 increasing from bottom to top.
Cursor Position Z	Float	The position of the crosshair on the block, from 0 to 1 increasing from north to south.
Inside block	Boolean	True when the player's head is inside of a block.
*/

#[derive(Clone, Serialize)]
pub struct PlayerBlockPlace {
    hand: i32,
    location: (i32, i32, i32),
    face: i32,
    cursor_pos_x: f32,
    cursor_pos_y: f32,
    cursor_pos_z: f32,
    inside_block: bool,
}

#[async_trait::async_trait]
impl Parsable for PlayerBlockPlace {
    fn default() -> Self {
        Self {
            hand: 0,
            location: (0, 0, 0),
            face: 0,
            cursor_pos_x: 0.0,
            cursor_pos_y: 0.0,
            cursor_pos_z: 0.0,
            inside_block: false,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.hand = packet.decode_varint()?;
        self.location = packet.decode_position()?;
        self.face = packet.decode_varint()?;
        self.cursor_pos_x = packet.decode_float()?;
        self.cursor_pos_y = packet.decode_float()?;
        self.cursor_pos_z = packet.decode_float()?;
        self.inside_block = packet.decode_bool()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {:?} {} {} {} {} {}",
            self.hand,
            self.location,
            self.face,
            self.cursor_pos_x,
            self.cursor_pos_y,
            self.cursor_pos_z,
            self.inside_block
        )
    }

    fn packet_editing(&self) -> bool {
        true
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
                    raw_packet.encode_varint(self.hand);
                    raw_packet.encode_position(self.location);
                    raw_packet.encode_varint(self.face);
                    raw_packet.encode_float(self.cursor_pos_x);
                    raw_packet.encode_float(self.cursor_pos_y);
                    raw_packet.encode_float(self.cursor_pos_z);
                    raw_packet.encode_bool(self.inside_block);
                    raw_packet
                },
                fid_to_pid(Fid::PlayerBlockPlace),
            ),
            Direction::Serverbound,
        )]);
    }
}
