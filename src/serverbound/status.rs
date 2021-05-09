use crate::{parsable::Parsable, raw_packet::RawPacket};

#[derive(Clone)]
pub struct StatusRequest {}

impl Parsable for StatusRequest {
    fn empty() -> Self {
        Self {}
    }

    fn parse_packet(&mut self, mut _packet: RawPacket) -> Result<(), ()> {
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("",)
    }
}

#[derive(Clone)]
pub struct StatusPing {
    payload: i64,
}

impl Parsable for StatusPing {
    fn empty() -> Self {
        Self { payload: 0 }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.payload = packet.decode_long()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!("{}", self.payload)
    }
}
