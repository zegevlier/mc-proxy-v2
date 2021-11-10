use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
    packet::Packet,
    parsable::Parsable,
    raw_packet::RawPacket,
    types::Uuid,
    Direction, EventHandler, SharedState,
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
enum ChatMessagePosition {
    Chat,
    SystemMessage,
    GameInfo,
}

#[derive(Clone, Serialize)]
pub struct ChatMessageClientbound {
    data: String,
    position: ChatMessagePosition,
    sender: Uuid,
}

#[async_trait::async_trait]
impl Parsable for ChatMessageClientbound {
    fn default() -> Self {
        Self {
            data: String::new(),
            position: ChatMessagePosition::Chat,
            sender: Uuid::from(0),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.data = packet.decode_chat()?;
        self.position = match packet.decode_byte()? {
            0 => ChatMessagePosition::Chat,
            1 => ChatMessagePosition::SystemMessage,
            2 => ChatMessagePosition::GameInfo,
            _ => return Err(()),
        };
        self.sender = packet.decode_uuid()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{} {:?} {}", self.data, self.position, self.sender)
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
        if self.data == "{\"text\":\"§d[§5§lF§d][§5FearRP §d-\\u003e §5zegevlier§d]§r now\"}"
        {
            Ok(vec![(
                {
                    let mut raw_packet = RawPacket::new();
                    raw_packet.encode_string("/buy".to_string());
                    Packet::from(raw_packet, 0x03)
                },
                Direction::Serverbound,
            )])
        } else if self.data == "{\"text\":\"§d[§5§lF§d][§5FearRP §d-\\u003e §5zegevlier§d]§r hi\"}"
        {
            Ok(vec![(
                {
                    Packet::from(
                        {
                            let mut raw_packet = RawPacket::new();
                            raw_packet.encode_varint(0);
                            raw_packet.encode_position((1820, 50, 1068));
                            raw_packet.encode_varint(4);
                            raw_packet.encode_float(0.0);
                            raw_packet.encode_float(0.5898391);
                            raw_packet.encode_float(0.51513046);
                            raw_packet.encode_bool(false);
                            raw_packet
                        },
                        fid_to_pid(Fid::PlayerBlockPlace),
                    )
                },
                Direction::Serverbound,
            )])
        } else {
            Ok(vec![])
        }
    }
}
