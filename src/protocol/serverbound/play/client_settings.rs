use crate::{parsable::Parsable, raw_packet::RawPacket};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Clone, Debug, Serialize)]
enum Hand {
    Left,
    Right,
}

#[derive(Clone, Serialize)]
pub struct ClientSettings {
    locale: String,
    view_distance: i8,
    chat_mode: ChatMode,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: Hand,
}

#[async_trait::async_trait]
impl Parsable for ClientSettings {
    fn default() -> Self {
        Self {
            locale: String::new(),
            view_distance: 0,
            chat_mode: ChatMode::Enabled,
            chat_colors: true,
            displayed_skin_parts: 0,
            main_hand: Hand::Right,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.locale = packet.decode_string()?;
        self.view_distance = packet.decode_byte()?;
        self.chat_mode = match packet.decode_varint()? {
            0 => ChatMode::Enabled,
            1 => ChatMode::CommandsOnly,
            2 => ChatMode::Hidden,
            _ => return Err(()),
        };
        self.chat_colors = packet.decode_bool()?;
        self.displayed_skin_parts = packet.decode_ubyte()?;
        self.main_hand = match packet.decode_varint()? {
            0 => Hand::Left,
            1 => Hand::Right,
            _ => return Err(()),
        };
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {:?} {} {} {:?}",
            self.locale,
            self.view_distance,
            self.chat_mode,
            self.chat_colors,
            self.displayed_skin_parts,
            self.main_hand
        )
    }
}
