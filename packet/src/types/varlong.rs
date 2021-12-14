#[macro_export]
macro_rules! varlong {
    ($v:expr) => {
        Varlong::from($v)
    };
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Varlong {
    value: i64,
}

impl Varlong {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Default for Varlong {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl crate::ProtoEnc for Varlong {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        let mut value = u64::from_le_bytes(self.value.to_le_bytes());
        loop {
            let mut temp: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;
            }
            p.push(temp);
            if value == 0 {
                break;
            }
        }
        Ok(())
    }
}

impl crate::ProtoDec for Varlong {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Varlong>
    where
        Self: Sized,
    {
        let mut num_read = 0;
        let mut result: i64 = 0;
        let mut read: u8;
        loop {
            read = p.read(1)?[0];
            let value: i64 = (read & 0x7F) as i64;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 10 {
                return Err(crate::Error::VarnumTooBig);
            }
            if (read & 0x80) == 0 {
                break;
            }
        }
        Ok(result.into())
    }

    fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
        *self = Self::decode_ret(p)?;
        Ok(())
    }
}

impl From<i64> for Varlong {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<Varlong> for i64 {
    fn from(value: Varlong) -> Self {
        value.value
    }
}

impl From<usize> for Varlong {
    fn from(value: usize) -> Self {
        Self::new(value as i64)
    }
}

impl From<Varlong> for usize {
    fn from(value: Varlong) -> Self {
        value.value as usize
    }
}

impl Varlong {
    pub fn to<T>(&self) -> T
    where
        T: From<Varlong>,
    {
        T::from(*self)
    }
}

impl std::cmp::Ord for Varlong {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl std::cmp::PartialEq<i64> for Varlong {
    fn eq(&self, other: &i64) -> bool {
        self.value.eq(&other)
    }
}

impl std::cmp::PartialOrd<i64> for Varlong {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other)
    }
}

impl std::fmt::Display for Varlong {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl crate::SizedDefault for Varlong {
    fn default() -> Self {
        Self { value: 0 }
    }
}
