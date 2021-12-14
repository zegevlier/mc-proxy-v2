#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BoolPrefixedOption<T> {
    pub v: Option<T>,
}

impl<T: crate::ProtoEnc> crate::ProtoEnc for BoolPrefixedOption<T> {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        match self.v {
            Some(ref v) => {
                p.encode(&true)?;
                p.encode(v)?;
            }
            None => {
                p.encode(&false)?;
            }
        }

        Ok(())
    }
}

impl<T: crate::ProtoDec> crate::ProtoDec for BoolPrefixedOption<T> {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        match p.decode::<bool>()? {
            true => {
                let v = p.decode::<T>()?;
                Ok(BoolPrefixedOption { v: Some(v) })
            }
            false => Ok(BoolPrefixedOption { v: None }),
        }
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        match p.decode::<bool>()? {
            true => {
                let v = p.decode::<T>()?;
                self.v = Some(v);
            }
            false => {
                self.v = None;
            }
        }
        Ok(())
    }
}

impl<T> std::fmt::Display for BoolPrefixedOption<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.v)
    }
}

impl<T> From<Option<T>> for BoolPrefixedOption<T> {
    fn from(v: Option<T>) -> Self {
        Self { v }
    }
}

impl<T> Into<Option<T>> for BoolPrefixedOption<T> {
    fn into(self) -> Option<T> {
        self.v
    }
}

impl<T> crate::SizedDefault for BoolPrefixedOption<T> {
    fn default() -> Self {
        Self { v: None }
    }
}
