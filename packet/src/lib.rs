mod error;
mod packet;
mod raw_packet;
mod traits;
mod types;

pub use error::{Error, Result};
pub use packet::Packet;
pub use raw_packet::RawPacket;
pub use traits::{ProtoDec, ProtoEnc};

pub use types::{Uuid, VarInt};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
