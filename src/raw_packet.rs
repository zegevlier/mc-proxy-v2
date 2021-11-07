use serde::{de, ser};
use std::{
    convert::TryInto,
    io::{Read, Write},
};

use crate::types::{Slot, Uuid};

// RawPacket holds a raw (unparsed) packet.
#[derive(Debug, Clone)]
pub struct RawPacket {
    data: Vec<u8>,
}

impl Read for RawPacket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // (&mut self.data[..]).read(buf)
        let n = Read::read(&mut &self.data[0..], buf)?;
        self.data.drain(0..n);
        Ok(n)
    }
}

impl Write for RawPacket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.push_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl RawPacket {
    pub fn new() -> RawPacket {
        RawPacket { data: Vec::new() }
    }

    pub fn from(packet_data: Vec<u8>) -> RawPacket {
        RawPacket { data: packet_data }
    }

    pub fn push(&mut self, data: u8) {
        self.data.push(data)
    }

    pub fn push_vec(&mut self, mut data: Vec<u8>) {
        self.data.append(&mut data)
    }

    pub fn push_slice(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn get_vec(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn clear(&mut self) {
        self.data = Vec::new();
    }

    pub fn read(&mut self, amount: usize) -> Result<Vec<u8>, ()> {
        if self.data.len() < amount {
            return Err(());
        }
        let to_be_returned = self.data.drain(0..amount);
        let read_value = to_be_returned.collect::<Vec<u8>>();
        Ok(read_value)
    }

    pub fn set(&mut self, value: Vec<u8>) {
        self.data = value;
    }

    pub fn prepend_length(&mut self) {
        let mut prepending = Self::new();
        prepending.encode_varint(self.data.len() as i32);
        prepending.push_vec(self.data.clone());
        self.set(prepending.get_vec());
    }

    pub fn decode_bool(&mut self) -> Result<bool, ()> {
        Ok(match self.read(1)?[0] {
            0x00 => false,
            0x01 => true,
            _ => return Err(()),
        })
    }

    pub fn decode_byte(&mut self) -> Result<i8, ()> {
        Ok(i8::from_be_bytes(self.read(1)?.try_into().unwrap()))
    }

    pub fn decode_ubyte(&mut self) -> Result<u8, ()> {
        Ok(self.read(1)?[0])
    }

    pub fn decode_short(&mut self) -> Result<i16, ()> {
        Ok(i16::from_be_bytes(self.read(2)?.try_into().unwrap()))
    }

    pub fn decode_ushort(&mut self) -> Result<u16, ()> {
        Ok(u16::from_be_bytes(self.read(2)?.try_into().unwrap()))
    }

    pub fn decode_int(&mut self) -> Result<i32, ()> {
        Ok(i32::from_be_bytes(self.read(4)?.try_into().unwrap()))
    }

    pub fn decode_long(&mut self) -> Result<i64, ()> {
        Ok(i64::from_be_bytes(self.read(8)?.try_into().unwrap()))
    }

    pub fn decode_ulong(&mut self) -> Result<u64, ()> {
        Ok(u64::from_le_bytes(self.read(8)?.try_into().unwrap()))
    }

    pub fn decode_float(&mut self) -> Result<f32, ()> {
        Ok(f32::from_be_bytes(self.read(4)?.try_into().unwrap()))
    }

    pub fn decode_double(&mut self) -> Result<f64, ()> {
        Ok(f64::from_be_bytes(self.read(8)?.try_into().unwrap()))
    }

    pub fn decode_string(&mut self) -> Result<String, ()> {
        let string_length = self.decode_varint()?;
        Ok(String::from_utf8(self.read(string_length.try_into().unwrap())?).unwrap())
    }

    pub fn decode_chat(&mut self) -> Result<String, ()> {
        self.decode_string()
    }

    pub fn decode_identifier(&mut self) -> Result<String, ()> {
        self.decode_string()
    }

    pub fn decode_varint(&mut self) -> Result<i32, ()> {
        let mut num_read = 0;
        let mut result: i32 = 0;
        let mut read: u8;
        loop {
            read = self.read(1)?[0];
            let value: i32 = (read & 0x7F) as i32;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(());
            }
            if (read & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    pub fn decode_varlong(&mut self) -> Result<i64, ()> {
        let mut num_read = 0;
        let mut result: i64 = 0;
        let mut read: u8;
        loop {
            read = self.read(1)?[0];
            let value: i64 = (read & 0x7F) as i64;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 10 {
                return Err(());
            }
            if (read & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    pub fn decode_entity_metadata(&mut self) -> Result<(), ()> {
        // varies, not yet needed so not yet implemented.
        todo!()
    }

    pub fn decode_slot(&mut self) -> Result<Slot, ()> {
        let present = self.decode_bool()?;
        if present {
            let item_id = self.decode_varint()?;
            let item_count = self.decode_byte()?;
            let nbt = if self.get_vec()[0] == 0 {
                // DO NOT REMOVE, this is to make the empty NBT get not get parsed as another slot later.
                self.read(1)?;
                nbt::Blob::new()
            } else {
                self.decode_nbt_blob()?
            };
            Ok(Slot {
                present,
                item_count: Some(item_count),
                item_id: Some(item_id),
                nbt: Some(nbt),
            })
        } else {
            Ok(Slot {
                present,
                item_count: None,
                item_id: None,
                nbt: None,
            })
        }
    }

    pub fn decode_nbt<T>(&mut self) -> Result<T, ()>
    where
        T: de::DeserializeOwned,
    {
        Ok(nbt::from_reader(self).unwrap())
    }

    pub fn decode_nbt_blob(&mut self) -> Result<nbt::Blob, ()> {
        Ok(nbt::Blob::from_reader(self).unwrap())
    }

    pub fn decode_position(&mut self) -> Result<(i32, i32, i32), ()> {
        let val = i64::from_be_bytes(self.read(8)?.try_into().unwrap());
        Ok((
            (val >> 38) as i32,
            (val & 0xFFF) as i32,
            (val << 26 >> 38) as i32,
        ))
    }

    pub fn decode_angle(&mut self) -> Result<u8, ()> {
        Ok(self.read(1)?[0])
    }

    pub fn decode_uuid(&mut self) -> Result<Uuid, ()> {
        Ok(Uuid::from(u128::from_be_bytes(
            self.read(16)?.try_into().unwrap(),
        )))
    }

    pub fn encode_bool(&mut self, value: bool) {
        self.push(match value {
            true => 0x01,
            false => 0x00,
        });
    }

    pub fn encode_byte(&mut self, value: i8) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_ubyte(&mut self, value: u8) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_short(&mut self, value: i16) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_ushort(&mut self, value: u16) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_int(&mut self, value: i32) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_long(&mut self, value: i64) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_ulong(&mut self, value: u64) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_float(&mut self, value: f32) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_double(&mut self, value: f64) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_string(&mut self, message: String) {
        self.encode_varint(message.len() as i32);
        self.push_slice(message.as_bytes());
    }

    pub fn encode_chat(&mut self, message: String) {
        self.encode_string(message);
    }

    pub fn encode_identifier(&mut self, message: String) {
        self.encode_string(message);
    }

    pub fn encode_varint(&mut self, v: i32) {
        let mut value = u32::from_le_bytes(v.to_le_bytes());
        loop {
            let mut temp: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;
            }
            self.push(temp);
            if value == 0 {
                break;
            }
        }
    }

    pub fn encode_varlong(&mut self, v: i64) {
        let mut value = u64::from_le_bytes(v.to_le_bytes());
        loop {
            let mut temp: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;
            }
            self.push(temp);
            if value == 0 {
                break;
            }
        }
    }

    pub fn encode_entity_metadata(&mut self) {
        // varies, not yet needed so not yet implemented.
        todo!()
    }

    pub fn encode_slot(&mut self, data: Slot) {
        self.encode_bool(data.present);
        if data.present {
            self.encode_varint(data.item_id.unwrap());
            self.encode_byte(data.item_count.unwrap());
            if data.nbt != Some(nbt::Blob::new()) {
                self.encode_nbt_blob(data.nbt.unwrap());
            } else {
                self.push(0);
            }
        }
    }

    pub fn encode_nbt<T>(&mut self, data: T)
    where
        T: ser::Serialize,
    {
        nbt::to_writer(self, &data, None).unwrap();
    }

    pub fn encode_nbt_blob(&mut self, data: nbt::Blob) {
        data.to_writer(self).unwrap()
    }

    pub fn encode_position(&mut self, data: (i32, i32, i32)) {
        let result = ((data.0 as u64 & 0x3FFFFFF) << 38)
            | ((data.2 as u64 & 0x3FFFFFF) << 12)
            | (data.1 as u64 & 0xFFF);
        self.push_slice(&result.to_be_bytes())
    }

    pub fn encode_angle(&mut self) {
        todo!()
    }

    pub fn encode_uuid(&mut self, value: u128) {
        self.push_slice(&value.to_be_bytes());
    }
}

impl Default for RawPacket {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Add a test for all the types a packet can parse.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint() {
        let mut packet = RawPacket::new();
        let values = vec![
            // Gotten from wiki.vg
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xff, 0xff, 0x7f], 2097151),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (vec![0xff, 0xff, 0xff, 0xff, 0x0f], -1),
            (vec![0x80, 0x80, 0x80, 0x80, 0x08], -2147483648),
        ];
        for (p, v) in values {
            packet.set(p);
            assert_eq!(packet.decode_varint().unwrap(), v);
            packet.clear()
        }
    }

    #[test]
    fn test_varint_writing() {
        let mut packet = RawPacket::new();
        let values = vec![
            // Gotten from wiki.vg
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xff, 0xff, 0x7f], 2097151),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (vec![0xff, 0xff, 0xff, 0xff, 0x0f], -1),
            (vec![0x80, 0x80, 0x80, 0x80, 0x08], -2147483648),
        ];
        for (p, v) in values {
            packet.encode_varint(v);
            assert_eq!(packet.get_vec(), p);
            packet.clear()
        }
    }

    #[test]
    fn test_varlong() {
        let mut packet = RawPacket::new();
        let values = vec![
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
                9223372036854775807,
            ),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
                -1,
            ),
            (
                vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
                -2147483648,
            ),
            (
                vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
                -9223372036854775808,
            ),
        ];
        for (p, v) in values {
            packet.set(p);
            assert_eq!(packet.decode_varlong().unwrap(), v);
            packet.clear()
        }
    }

    #[test]
    fn test_varlong_writing() {
        let mut packet = RawPacket::new();
        let values = vec![
            // Gotten from wiki.vg
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
                9223372036854775807,
            ),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
                -1,
            ),
            (
                vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
                -2147483648,
            ),
            (
                vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
                -9223372036854775808,
            ),
        ];
        for (p, v) in values {
            packet.encode_varlong(v);
            assert_eq!(packet.get_vec(), p);
            packet.clear()
        }
    }

    #[test]
    fn test_bool() {
        let mut packet = RawPacket::new();
        let values = vec![true, false];
        for v in values {
            packet.encode_bool(v);
            assert_eq!(packet.decode_bool().unwrap(), v);
            packet.clear()
        }
    }

    #[test]
    fn test_byte() {
        let mut packet = RawPacket::new();
        let values = vec![0, 1, -1, -128, 127];
        for v in values {
            packet.encode_byte(v);
            assert_eq!(packet.decode_byte().unwrap(), v);
            packet.clear()
        }
    }

    #[test]
    fn test_read() {
        let mut packet = RawPacket::new();
        packet.encode_bool(true);
        test_read_helper(&mut packet);
        let packet_vec: Vec<u8> = Vec::new();
        assert_eq!(packet.get_vec(), packet_vec);
    }

    fn test_read_helper(r: &mut impl Read) {
        let buf = &mut [0_u8; 1];
        r.read_exact(buf).unwrap();
    }

    #[test]
    fn test_nbt_small() {
        #[derive(Debug, PartialEq, serde::Deserialize)]
        pub struct Small1 {
            name: String,
        }

        let test_data = vec![
            0x0A, 0x00, 0x0B, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64,
            0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x09, 0x42, 0x61, 0x6E, 0x61, 0x6E,
            0x72, 0x61, 0x6D, 0x61, 0x00,
        ];
        let nbt = Small1 {
            name: "Bananrama".to_string(),
        };

        let mut raw_packet = RawPacket::new();
        raw_packet.push_vec(test_data);
        let nbt_data: Small1 = raw_packet.decode_nbt().unwrap();
        assert_eq!(raw_packet.len(), 0);
        assert_eq!(nbt_data, nbt);
    }
}
