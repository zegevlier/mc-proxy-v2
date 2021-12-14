use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    Eof,
    VarnumTooBig,
    InvalidVarintEnum,
    PacketIdNotSet,
    UnknownEnum,
    InvalidData,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::VarnumTooBig => formatter.write_str("varint too big"),
            Error::InvalidVarintEnum => formatter.write_str("invalid varint enum"),
            Error::PacketIdNotSet => formatter.write_str("packet id not set"),
            Error::UnknownEnum => formatter.write_str("unknown enum"),
            Error::InvalidData => formatter.write_str("invalid data"),
        }
    }
}

impl From<Error> for () {
    fn from(_: Error) -> Self {}
}

impl std::error::Error for Error {}
