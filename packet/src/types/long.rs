use std::convert::TryInto;

impl crate::ProtoEnc for i64 {
    fn encode(&self, p: &mut crate::RawPacket) {
        p.push_slice(&self.to_be_bytes());
    }
}

impl crate::ProtoDec for i64 {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(i64::from_be_bytes(p.read(8)?.try_into().unwrap()))
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl crate::SafeDefault for i64 {
    fn default() -> Self {
        0
    }
}
