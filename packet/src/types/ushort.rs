use std::convert::TryInto;

impl crate::ProtoEnc for u16 {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.push_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl crate::ProtoDec for u16 {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(u16::from_be_bytes(p.read(2)?.try_into().unwrap()))
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl crate::SafeDefault for u16 {
    fn default() -> Self {
        0
    }
}
