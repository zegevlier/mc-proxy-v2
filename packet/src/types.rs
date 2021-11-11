mod varint;
pub use varint::VarInt;
mod uuid;
pub use uuid::Uuid;

mod long;
mod string;
mod ushort;

pub trait SafeDefault {
    fn default() -> Self
    where
        Self: Sized;
}
