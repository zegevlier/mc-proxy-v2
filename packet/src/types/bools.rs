impl crate::ProtoEnc for bool {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.push(match *self {
            true => 0x01,
            false => 0x00,
        });
        Ok(())
    }
}

impl crate::ProtoDec for bool {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(match p.read(1)?[0] {
            0x01 => true,
            0x00 => false,
            _ => return Err(crate::Error::InvalidData),
        })
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl crate::SafeDefault for bool {
    fn default() -> Self {
        false
    }
}
