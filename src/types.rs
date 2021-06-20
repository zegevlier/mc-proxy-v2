use crate::{cipher::Cipher, DataQueue};
use std::{fmt, sync::Arc};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Clone)]
pub struct SharedState {
    pub compress: u32,
    pub state: State,
    pub secret_key: [u8; 16],
}

impl SharedState {
    pub fn new() -> SharedState {
        Self {
            compress: 0,
            state: State::Handshaking,
            secret_key: [0; 16],
        }
    }

    pub fn set(&mut self, new_state: SharedState) {
        self.compress = new_state.compress;
        self.state = new_state.state.clone();
        self.secret_key = new_state.secret_key;
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}

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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    Serverbound,
    Clientbound,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Queues {
    pub client_proxy: Arc<DataQueue>,
    pub proxy_client: Arc<DataQueue>,
    pub server_proxy: Arc<DataQueue>,
    pub proxy_server: Arc<DataQueue>,
}
