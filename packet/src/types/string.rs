use crate::{varint, VarInt};

impl crate::ProtoEnc for String {
    fn encode(&self, p: &mut crate::RawPacket) {
        p.encode(&varint!(self.len()));
        p.push_slice(self.as_bytes());
    }
}

impl crate::ProtoDec for String {
    fn decode(p: &mut crate::RawPacket) -> crate::Result<String>
    where
        Self: Sized,
    {
        let string_length = p.decode::<VarInt>()?.into();
        Ok(String::from_utf8(p.read(string_length)?).unwrap())
    }
}
