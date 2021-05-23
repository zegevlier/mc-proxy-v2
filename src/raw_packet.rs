use std::convert::TryInto;

// RawPacket holds a raw (unparsed) packet.
#[derive(Debug, Clone)]
pub struct RawPacket {
    data: Vec<u8>,
}

// TODO: Make this type also hold a packet ID, and be able to generate packets.
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
            let value: i32 = (read & 0b01111111) as i32;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(());
            }
            if (read & 0b10000000) == 0 {
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
            let value: i64 = (read & 0b01111111) as i64;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 10 {
                return Err(());
            }
            if (read & 0b10000000) == 0 {
                break;
            }
        }
        Ok(result)
    }

    pub fn decode_entity_metadata(&mut self) -> Result<(), ()> {
        // varies, not yet needed so not yet implemented.
        todo!()
    }

    pub fn decode_slot(&mut self) -> Result<(), ()> {
        // requires NTB parsing
        todo!()
    }

    pub fn decode_nbt_tag(&mut self) -> Result<(), ()> {
        // Requires a *lot* of work and is not yet needed, so TODO.
        todo!()
    }

    pub fn decode_position(&mut self) -> Result<(i64, i64, i64), ()> {
        let val = i64::from_be_bytes(self.read(8)?.try_into().unwrap());
        let mut x = val >> 38;
        let mut y = val & 0xFFF;
        let mut z = val << 26 >> 38;

        if x >= 2 ^ 25 {
            x -= 2 ^ 26
        }
        if y >= 2 ^ 11 {
            y -= 2 ^ 12
        }
        if z >= 2 ^ 25 {
            z -= 2 ^ 26
        }

        Ok((x, y, z))
    }

    pub fn decode_angle(&mut self) -> Result<u8, ()> {
        Ok(self.read(1)?[0])
    }

    pub fn decode_uuid(&mut self) -> Result<u128, ()> {
        Ok(u128::from_be_bytes(self.read(16)?.try_into().unwrap()))
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

    pub fn encode_short(&mut self) {
        todo!()
    }

    pub fn encode_ushort(&mut self, value: u16) {
        self.push_slice(&value.to_be_bytes());
    }

    pub fn encode_int(&mut self) {
        todo!()
    }

    pub fn encode_long(&mut self) {
        todo!()
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

    pub fn encode_chat(&mut self) {
        todo!()
    }

    pub fn encode_identifier(&mut self) {
        todo!()
    }

    pub fn encode_varint(&mut self, v: i32) {
        let mut value = u32::from_le_bytes(v.to_le_bytes());
        loop {
            let mut temp: u8 = (value & 0b01111111) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0b10000000;
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
            let mut temp: u8 = (value & 0b01111111) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0b10000000;
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

    pub fn encode_slot(&mut self) {
        // requires NTB parsing
        todo!()
    }

    pub fn encode_nbt_tag(&mut self) {
        // Requires a *lot* of work and is not yet needed, so TODO.
        todo!()
    }

    pub fn encode_position(&mut self) {
        todo!()
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
}
