#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Identifier {
    value: String,
}

impl crate::ProtoEnc for Identifier {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.encode(&self.value)
    }
}

impl crate::ProtoDec for Identifier {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Identifier { value: p.decode()? })
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Identifier { value }
    }
}

impl From<String> for Identifier {
    fn from(v: String) -> Self {
        Self { value: v }
    }
}

impl crate::SizedDefault for Identifier {
    fn default() -> Self {
        Self {
            value: "".to_string(),
        }
    }
}
