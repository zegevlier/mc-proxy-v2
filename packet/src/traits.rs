use crate::{error::Result, Packet, RawPacket, SizedDefault};

pub trait ProtoEnc {
    fn encode(&self, p: &mut RawPacket) -> Result<()>;

    fn encode_packet(&self) -> Result<Packet> {
        unimplemented!()
    }
}

pub trait ProtoDec: SizedDefault {
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
