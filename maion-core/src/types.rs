use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, serde::Serialize)]
pub enum Direction {
    Serverbound,
    Clientbound,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
 