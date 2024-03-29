// use aes::cipher::AsyncStreamCipher;
use aes::Aes128;
mod cfb8;
use cfb8::{
    cipher::{AsyncStreamCipher, NewCipher},
    Cfb8,
};

type AesCfb8 = Cfb8<Aes128>;

#[derive(Clone)]
pub struct Cipher {
    pub encryptor: Option<AesCfb8>,
}

impl Cipher {
    pub fn new() -> Self {
        Self { encryptor: None }
    }

    pub fn decrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        match &mut self.encryptor {
            Some(encryptor) => {
                encryptor.decrypt(data.as_mut_slice());
                data
            }
            None => data,
        }
    }

    pub fn encrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        match &mut self.encryptor {
            Some(encryptor) => {
                encryptor.encrypt(data.as_mut_slice());
                data
            }
            None => data,
        }
    }

    pub fn enable(&mut self, key: &[u8]) {
        let cipher = AesCfb8::new_from_slices(key, key).unwrap();
        self.encryptor = Some(cipher);
    }

    pub fn disable(&mut self) {
        self.encryptor = None;
    }
}

impl Default for Cipher {
    fn default() -> Self {
        Self::new()
    }
}
