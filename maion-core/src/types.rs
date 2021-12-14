use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, serde::Serialize)]
pub enum Direction {
    Serverbound,
    Clientbound,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

packet::varint_enum!(
    State; Copy; {
        0 = Handshaking,
        1 = Status,
        2 = Login,
        3 = Play,
    }
);

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
