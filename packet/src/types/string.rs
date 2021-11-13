use crate::{varint, Varint};

impl crate::ProtoEnc for String {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.encode(&varint!(self.len()))?;
        p.push_slice(self.as_bytes());
        Ok(())
    }
}

impl crate::ProtoDec for String {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<String>
    where
        Self: Sized,
    {
        let string_length = p.decode::<Varint>()?.into();
        Ok(String::from_utf8(p.read(string_length)?).unwrap())
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl crate::SafeDefault for String {
    fn default() -> Self {
        "".to_string()
    }
}
