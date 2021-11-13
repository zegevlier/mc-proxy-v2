mod error;
mod packet;
mod raw_packet;
mod traits;
mod types;

pub use self::packet::Packet;
pub use error::{Error, Result};
pub use raw_packet::RawPacket;
pub use traits::{ProtoDec, ProtoEnc};

pub use types::{SafeDefault, Uuid, Varint};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
