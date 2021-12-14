mod cfb8;
mod single;

pub use single::Cipher;

pub struct Ciphers {
    pub ps_cipher: Cipher,
    pub sp_cipher: Cipher,
}

impl Ciphers {
    pub fn new() -> Ciphers {
        Ciphers {
            ps_cipher: Cipher::new(),
            sp_cipher: Cipher::new(),
        }
    }
}

impl Default for Ciphers {
    fn default() -> Self {
        Self::new()
    }
}
