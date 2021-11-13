impl crate::ProtoEnc for Vec<u8> {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.push_slice(&self[..]);
        Ok(())
    }
}

impl crate::ProtoDec for Vec<u8> {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(p.get_vec())
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl crate::SafeDefault for Vec<u8> {
    fn default() -> Self {
        Vec::new()
    }
}
