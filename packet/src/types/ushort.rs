use std::convert::TryInto;

impl crate::ProtoEnc for u16 {
    fn encode(&self, p: &mut crate::RawPacket) {
        p.push_slice(&self.to_be_bytes());
    }
}

impl crate::ProtoDec for u16 {
    fn decode(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(u16::from_be_bytes(p.read(2)?.try_into().unwrap()))
    }
}
