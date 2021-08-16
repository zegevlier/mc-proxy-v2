use crate::{
    conf::Configuration, packet::Packet, parsable::Parsable, raw_packet::RawPacket, types::Uuid,
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
    fn empty() -> Self {
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
        status: SharedState,
        _plugins: &mut Vec<Box<dyn EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
        if self.data == "{\"text\":\"\",\"extra\":[{\"text\":\"[\",\"color\":\"dark_purple\"},{\"text\":\"F\",\"color\":\"light_purple\",\"bold\":true},{\"text\":\"] [\",\"color\":\"dark_purple\"},{\"text\":\"FearRP \",\"color\":\"light_purple\"},{\"text\":\"-\\u003e \",\"color\":\"dark_purple\"},{\"text\":\"zegevlier\",\"color\":\"light_purple\"},{\"text\":\"] \",\"color\":\"dark_purple\"},{\"text\":\"test\",\"color\":\"white\"}]}" {
            Ok((vec![({
                let mut raw_packet = RawPacket::new();
                raw_packet.encode_string("/qav callback".to_string());
                Packet::from(raw_packet, 0x03)
            }, Direction::Serverbound)], status))
        } else {
            Ok((vec![], status))
        }
    }
}
