use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone)]
pub struct LoginStart {
    username: String,
}

impl Parsable for LoginStart {
    fn empty() -> Self {
        Self {
            username: "".into(),
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.username = packet.decode_string()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        self.username.to_string()
    }
}
