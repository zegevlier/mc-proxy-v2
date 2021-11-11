use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    Eof,
    VarIntTooBig,
    InvalidVarintEnum,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::VarIntTooBig => formatter.write_str("varint too big"),
            Error::InvalidVarintEnum => formatter.write_str("invalid varint enum"),
        }
    }
}

impl From<Error> for () {
    fn from(_: Error) -> Self {
        ()
    }
}

impl std::error::Error for Error {}
