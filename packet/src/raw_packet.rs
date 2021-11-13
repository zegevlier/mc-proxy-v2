use std::io::{Read, Write};

use crate::{
    error::{Error, Result},
    ProtoDec, ProtoEnc, Varint,
};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct RawPacket {
    data: Vec<u8>,
}

impl Read for RawPacket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_vec(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn get_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn clear(&mut self) {
        self.data = Vec::new();
    }

    pub fn read(&mut self, amount: usize) -> Result<Vec<u8>> {
        if self.data.len() < amount {
            return Err(Error::Eof);
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
        prepending.encode(&Varint::from(self.data.len())).unwrap();
        prepending.push_vec(self.data.clone());
        self.set(prepending.get_vec());
    }

    pub fn encode(&mut self, value: &dyn ProtoEnc) -> crate::Result<()> {
        value.encode(self)
    }

    pub fn decode<T>(&mut self) -> Result<T>
    where
        T: Sized + ProtoDec,
    {
        T::decode_ret(self)
    }
}

impl Default for RawPacket {
    fn default() -> Self {
        Self::new()
    }
}
