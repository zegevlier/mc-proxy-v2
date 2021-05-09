// For now, I don't want the cd to fail because of this. Should be removed later
#![allow(dead_code)]
use crate::RawPacket;

pub struct Packet {
    raw_packet: RawPacket,
    pid: Option<i32>,
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            raw_packet: RawPacket::new(),
            pid: None,
        }
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::new()
    }
}
