use crate::cipher::Cipher;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Clone)]
pub struct SharedState {
    pub ps_cipher: Cipher,
    pub sp_cipher: Cipher,
    pub compress: u32,
    pub state: State,
    pub secret_key: [u8; 16],
}

impl SharedState {
    pub fn new() -> SharedState {
        Self {
            ps_cipher: Cipher::new(),
            sp_cipher: Cipher::new(),
            compress: 0,
            state: State::Handshaking,
            secret_key: [0; 16],
        }
    }

    pub fn set(&mut self, new_state: SharedState) {
        self.ps_cipher = new_state.ps_cipher;
        self.sp_cipher = new_state.sp_cipher;
        self.compress = new_state.compress;
        self.state = new_state.state;
        self.secret_key = new_state.secret_key;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    Serverbound,
    Clientbound,
}

impl Direction {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
