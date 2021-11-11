use crate::{error::Result, RawPacket};

pub trait ProtoEnc {
    fn encode(&self, p: &mut RawPacket);
}

pub trait ProtoDec {
    fn decode(p: &mut RawPacket) -> Result<Self>
    where
        Self: Sized;
}
