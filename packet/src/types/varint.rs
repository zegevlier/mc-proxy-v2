#[macro_export]
macro_rules! varint {
    ($v:expr) => {
        Varint::from($v)
    };
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Varint {
    value: i32,
}

impl Varint {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Default for Varint {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl crate::ProtoEnc for Varint {
    fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
        let mut value = u32::from_le_bytes(self.value.to_le_bytes());
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

impl crate::ProtoDec for Varint {
    fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Varint>
    where
        Self: Sized,
    {
        let mut num_read = 0;
        let mut result: i32 = 0;
        let mut read: u8;
        loop {
            read = p.read(1)?[0];
            let value: i32 = (read & 0x7F) as i32;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(crate::Error::VarIntTooBig);
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

impl From<i32> for Varint {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<Varint> for i32 {
    fn from(value: Varint) -> Self {
        value.value
    }
}

impl From<usize> for Varint {
    fn from(value: usize) -> Self {
        Self::new(value as i32)
    }
}

impl From<Varint> for usize {
    fn from(value: Varint) -> Self {
        value.value as usize
    }
}

impl Varint {
    pub fn to<T>(&self) -> T
    where
        T: From<Varint>,
    {
        T::from(*self)
    }
}

impl std::cmp::Ord for Varint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl std::cmp::PartialEq<i32> for Varint {
    fn eq(&self, other: &i32) -> bool {
        self.value.eq(&other)
    }
}

impl std::cmp::PartialOrd<i32> for Varint {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other)
    }
}

impl std::fmt::Display for Varint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl crate::SafeDefault for Varint {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::RawPacket;

    use super::*;

    fn get_test_values() -> Vec<(Vec<u8>, i32)> {
        vec![
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xff, 0xff, 0x7f], 2097151),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (vec![0xff, 0xff, 0xff, 0xff, 0x0f], -1),
            (vec![0x80, 0x80, 0x80, 0x80, 0x08], -2147483648),
        ]
    }

    #[test]
    fn decode() {
        let mut packet = RawPacket::new();
        for (p, v) in get_test_values() {
            packet.set(p);
            assert_eq!(packet.decode(), Ok(Varint::from(v)));
            packet.clear()
        }
    }

    #[test]
    fn encode() {
        let mut packet = RawPacket::new();
        for (p, v) in get_test_values() {
            packet.encode(&Varint::from(v)).unwrap();
            assert_eq!(packet.get_vec(), p);
            packet.clear()
        }
    }
}
