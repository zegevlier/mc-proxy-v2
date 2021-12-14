#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Chat {
    value: String,
}

impl crate::ProtoEnc for Chat {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.encode(&self.value)
    }
}

impl crate::ProtoDec for Chat {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Chat { value: p.decode()? })
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl std::fmt::Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Chat {
    pub fn new(value: String) -> Self {
        Chat { value }
    }

    pub fn get_string(&self) -> &String {
        &self.value
    }
}

impl From<String> for Chat {
    fn from(v: String) -> Self {
        Self { value: v }
    }
}

impl crate::SizedDefault for Chat {
    fn default() -> Self {
        Self {
            value: "".to_string(),
        }
    }
}
