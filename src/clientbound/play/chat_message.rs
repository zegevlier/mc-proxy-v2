use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone, Debug)]
enum ChatMessagePosition {
    Chat,
    SystemMessage,
    GameInfo,
}

#[derive(Clone)]
pub struct ChatMessageClientbound {
    data: String,
    position: ChatMessagePosition,
    sender: u128,
}

impl Parsable for ChatMessageClientbound {
    fn empty() -> Self {
        Self {
            data: String::new(),
            position: ChatMessagePosition::Chat,
            sender: 0,
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
        format!("{} {:?} {:x}", self.data, self.position, self.sender)
    }
}
