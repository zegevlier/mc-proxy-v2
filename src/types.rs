use crate::cipher::Cipher;
use serde::{Serialize, Serializer};
use std::{fmt, sync::Arc};

pub type DataQueue = deadqueue::unlimited::Queue<Vec<u8>>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize)]
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
    pub access_token: String,
    pub uuid: String,
    pub server_ip: String,
    pub user_ip: String,
    pub connection_id: String,
}

impl SharedState {
    pub fn new() -> SharedState {
        Self {
            compress: 0,
            state: State::Handshaking,
            secret_key: [0; 16],
            access_token: String::new(),
            uuid: String::new(),
            server_ip: String::new(),
            user_ip: String::new(),
            connection_id: String::new(),
        }
    }

    pub fn set(&mut self, new_state: SharedState) {
        self.compress = new_state.compress;
        self.state = new_state.state;
        self.secret_key = new_state.secret_key;
        self.access_token = new_state.access_token;
        self.uuid = new_state.uuid;
        self.server_ip = new_state.server_ip;
        self.user_ip = new_state.user_ip;
        self.connection_id = new_state.connection_id;
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Slot {
    pub present: bool,
    pub item_id: Option<i32>,
    pub item_count: Option<i8>,
    pub nbt: Option<nbt::Blob>,
}

#[derive(Debug, Clone)]
pub struct Uuid {
    value: u128,
}

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:x}", self.value))
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.value)
    }
}

impl Uuid {
    pub fn from(v: u128) -> Self {
        Self { value: v }
    }
}
