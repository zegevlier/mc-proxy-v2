use serde::{Serialize, Serializer};
use std::convert::TryInto;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Uuid {
    value: u128,
}

impl crate::ProtoEnc for Uuid {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.push_slice(&self.value.to_be_bytes());
        Ok(())
    }
}

impl crate::ProtoDec for Uuid {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Uuid::from(u128::from_be_bytes(
            p.read(16)?.try_into().unwrap(),
        )))
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:x}", self.value))
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.value)
    }
}

impl Uuid {
    pub fn from(v: u128) -> Self {
        Self { value: v }
    }
}

impl crate::SizedDefault for Uuid {
    fn default() -> Self {
        Self { value: 0 }
    }
}
