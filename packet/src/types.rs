mod varint;
pub use varint::Varint;
mod uuid;
pub use uuid::Uuid;
mod chat;
pub use chat::Chat;
mod len_prefixed_vec;
pub use len_prefixed_vec::LenPrefixedVec;

mod bools;
mod nums;
mod string;
mod varint_enum;
mod vecs;

pub trait SafeDefault {
    fn default() -> Self
    where
        Self: Sized;
}
