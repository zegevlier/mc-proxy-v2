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

    pub fn from(raw_packet: RawPacket, pid: i32) -> Packet {
        Packet {
            raw_packet,
            pid: Some(pid),
        }
    }

    pub fn get_data_uncompressed(&self) -> Result<Vec<u8>, ()> {
        let mut pid_encoded = RawPacket::new();
        match self.pid {
            Some(pid) => pid_encoded.encode_varint(pid)?,
            None => return Err(()),
        }

        let mut data = RawPacket::new();
        data.encode_varint((self.raw_packet.len() + pid_encoded.len()) as i32)?;
        data.push_vec(pid_encoded.get_vec());
        data.push_vec(self.raw_packet.get_vec());

        Ok(data.get_vec())
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::new()
    }
}
