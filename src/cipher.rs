use aes::Aes128;
use cfb8::cipher::{NewStreamCipher, StreamCipher};
use cfb8::Cfb8;
use parking_lot::Mutex;
use std::sync::Arc;

type AesCfb8 = Cfb8<Aes128>;

#[derive(Clone)]
pub struct Cipher {
    encryptor: Option<Arc<Mutex<AesCfb8>>>,
}

impl Cipher {
    pub fn new() -> Self {
        Self { encryptor: None }
    }

    pub fn decrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        match &mut self.encryptor.take() {
            Some(encryptor) => {
                encryptor.lock().decrypt(data.as_mut_slice());
                data
            }
            None => data,
        }
    }

    pub fn encrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        match &mut self.encryptor.take() {
            Some(encryptor) => {
                encryptor.lock().encrypt(data.as_mut_slice());
                data
            }
            None => data,
        }
    }

    pub fn enable(&mut self, key: &[u8]) {
        let cipher = AesCfb8::new_var(key, key).unwrap();
        self.encryptor = Some(Arc::new(Mutex::new(cipher)));
    }

    pub fn disable(&mut self) {
        self.encryptor = None;
    }
}
