use crate::Varint;

pub trait LPVable {}
macro_rules! impl_lpv {
    ($($t:ty),* $(,)?) => {
        $(impl LPVable for Vec<$t> {})*
    };
}

impl_lpv! {
    Varint
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LenPrefixedVec<T> {
    pub v: Vec<T>,
}

impl<T> LenPrefixedVec<T> {
    pub fn len(&self) -> usize {
        self.v.len()
    }
}

impl crate::ProtoEnc for LenPrefixedVec<u8> {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.encode(&Varint::from(self.v.len()))?;
        p.push_slice(&self.v);
        Ok(())
    }
}

impl crate::ProtoDec for LenPrefixedVec<u8> {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let len: usize = p.decode::<Varint>()?.into();
        Ok(Self { v: p.read(len)? })
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        let len: Varint = p.decode()?;
        self.v = p.read(len.into())?;
        Ok(())
    }
}

impl<T: LPVable + crate::ProtoEnc> crate::ProtoEnc for LenPrefixedVec<T> {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        p.encode(&Varint::from(self.v.len()))?;
        for i in 0..self.v.len() {
            p.encode(&self.v[i])?;
        }
        Ok(())
    }
}

impl<T: LPVable + crate::ProtoDec> crate::ProtoDec for LenPrefixedVec<T> {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let len: usize = p.decode::<Varint>()?.into();
        let mut ret = Self {
            v: Vec::with_capacity(len.into()),
        };

        for _ in 0..len.into() {
            ret.v.push(p.decode()?);
        }
        Ok(ret)
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        let len: Varint = p.decode()?;
        for _ in 0..len.into() {
            self.v.push(p.decode()?);
        }
        Ok(())
    }
}

impl<T> std::fmt::Display for LenPrefixedVec<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.v)
    }
}

impl<T> From<Vec<T>> for LenPrefixedVec<T> {
    fn from(v: Vec<T>) -> Self {
        Self { v }
    }
}

impl<T> Into<Vec<T>> for LenPrefixedVec<T> {
    fn into(self) -> Vec<T> {
        self.v
    }
}

impl<T> crate::SafeDefault for LenPrefixedVec<T> {
    fn default() -> Self {
        Self { v: Vec::new() }
    }
}
