use std::convert::TryInto;

macro_rules! num {
    ($type:ty, $bytes:expr) => {
        impl crate::ProtoEnc for $type {
            fn encode(&self, p: &mut crate::RawPacket) -> crate::Result<()> {
                p.push_slice(&self.to_be_bytes());
                Ok(())
            }
        }

        impl crate::ProtoDec for $type {
            fn decode_ret(p: &mut crate::RawPacket) -> crate::Result<Self>
            where
                Self: Sized,
            {
                Ok(<$type>::from_be_bytes(p.read($bytes)?.try_into().unwrap()))
            }

            fn decode(&mut self, p: &mut crate::RawPacket) -> crate::Result<()> {
                *self = Self::decode_ret(p)?;
                Ok(())
            }
        }

        impl crate::SafeDefault for $type {
            fn default() -> Self {
                0
            }
        }
    };
}

num!(u8, 1);
num!(u16, 2);
num!(u32, 4);
num!(u64, 8);
num!(i8, 1);
num!(i16, 2);
num!(i32, 4);
num!(i64, 8);
