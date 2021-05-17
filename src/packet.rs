use miniz_oxide::deflate::compress_to_vec_zlib;

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
            Some(pid) => pid_encoded.encode_varint(pid),
            None => return Err(()),
        }

        let mut data = RawPacket::new();
        data.encode_varint((self.raw_packet.len() + pid_encoded.len()) as i32);
        data.push_vec(pid_encoded.get_vec());
        data.push_vec(self.raw_packet.get_vec());

        Ok(data.get_vec())
    }

    pub fn get_data_compressed(&self, compression_threshold: i32) -> Result<Vec<u8>, ()> {
        let mut pid_encoded = RawPacket::new();
        match self.pid {
            Some(pid) => pid_encoded.encode_varint(pid),
            None => return Err(()),
        }

        let mut data = RawPacket::new();
        data.push_vec(pid_encoded.get_vec());
        data.push_vec(self.raw_packet.get_vec());

        let data_length = if data.len() >= compression_threshold as usize {
            let dl = data.len();
            data.set(compress_to_vec_zlib(data.get_slice(), 6));
            dl
        } else {
            0
        };

        let mut data_length_encoded = RawPacket::new();
        data_length_encoded.encode_varint(data_length as i32);

        let mut return_data = RawPacket::new();
        return_data.encode_varint((data_length_encoded.len() + data.len()) as i32);
        return_data.push_vec(data_length_encoded.get_vec());
        return_data.push_vec(data.get_vec());

        Ok(return_data.get_vec())
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::new()
    }
}
