mod varint;
pub use varint::VarInt;
mod uuid;
pub use uuid::Uuid;

mod nums;
mod string;

pub trait SafeDefault {
    fn default() -> Self
    where
        Self: Sized;
}
