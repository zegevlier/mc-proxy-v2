use crate::{error::Result, RawPacket, SafeDefault};

pub trait ProtoEnc {
    fn encode(&self, p: &mut RawPacket);
}

pub trait ProtoDec: SafeDefault {
    fn decode_ret(p: &mut RawPacket) -> Result<Self>
    where
        Self: Sized,
    {
        let mut ret = Self::default();
        ret.decode(p)?;
        Ok(ret)
    }

    fn decode(&mut self, p: &mut RawPacket) -> Result<()>;
}
