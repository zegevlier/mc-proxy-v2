use std::{collections::HashMap, fmt};

use maplit::hashmap;

use crate::{
    clientbound,
    parsable::Parsable,
    serverbound,
    types::{Direction, State},
};

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Fid {
    Unparsable,
    Handshake,
    StatusResponse,
    StatusPong,
    StatusRequest,
    StatusPing,
    Disconnect,
    EncRequest,
    LoginSuccess,
    SetCompression,
    PluginRequest,
    LoginStart,
    EncResponse,
    PluginResponse,
    SpawnEntity,
    SpawnXpOrb,
    SpawnLivingEntity,
    SpawnPainting,
    SpawnPlayer,
    AckPlayerDigging,
    ChatMessageClientbound,
    TabCompleteClientbound,
    ChatMessageServerbound,
    ResourcePackSend,
}

impl fmt::Display for Fid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Functions {
    map: HashMap<Direction, HashMap<State, HashMap<i32, Fid>>>,
    list: HashMap<Fid, Box<dyn Parsable + Send + Sync>>,
}

impl Functions {
    fn new() -> Self {
        Self {
            map: hashmap! {
                Direction::Clientbound => hashmap! {
                    State::Handshaking => hashmap! {},
                    State::Status => hashmap! {
                        0x00 => Fid::StatusResponse,
                        0x01 => Fid::StatusPong
                    },
                    State::Login => hashmap! {
                        0x00 => Fid::Disconnect,
                        0x01 => Fid::EncRequest,
                        0x02 => Fid::LoginSuccess,
                        0x03 => Fid::SetCompression,
                        0x04 => Fid::PluginRequest
                    },
                    State::Play => hashmap! {
                        0x00 => Fid::SpawnEntity,
                        0x01 => Fid::SpawnXpOrb,
                        0x02 => Fid::SpawnLivingEntity,
                        0x03 => Fid::SpawnPainting,
                        0x04 => Fid::SpawnPlayer,
                        0x07 => Fid::AckPlayerDigging,
                        0x0E => Fid::ChatMessageClientbound,
                        0x0F => Fid::TabCompleteClientbound,
                        0x38 => Fid::ResourcePackSend,
                    },
                },
                Direction::Serverbound => hashmap! {
                    State::Handshaking => hashmap! {
                        0x00 => Fid::Handshake,
                    },
                    State::Status => hashmap! {
                        0x00 => Fid::StatusRequest,
                        0x01 => Fid::StatusPing,
                    },
                    State::Login => hashmap! {
                        0x00 => Fid::LoginStart,
                        0x01 => Fid::EncResponse,
                        0x02 => Fid::PluginResponse,
                    },
                    State::Play => hashmap! {
                        0x03 => Fid::ChatMessageServerbound,
                    },
                },

            },
            list: HashMap::new(),
        }
    }

    fn add(&mut self, id: Fid, func: Box<dyn Parsable + Send + Sync>) {
        self.list.insert(id, func);
    }

    pub fn get_name(&self, direction: &Direction, state: &State, pid: &i32) -> Option<&Fid> {
        self.map
            .get(direction)
            .unwrap()
            .get(state)
            .unwrap()
            .get(pid)
    }

    pub fn get(&self, id: &Fid) -> Option<Box<dyn Parsable + Send + Sync>> {
        self.list.get(id).cloned()
    }
}

pub fn get_functions() -> Functions {
    let mut functions = Functions::new();

    // Handshaking
    // Serverbound
    functions.add(
        Fid::Handshake,
        Box::new(serverbound::handshaking::Handshake::empty()),
    );

    // Status
    // Clientbound
    functions.add(
        Fid::StatusResponse,
        Box::new(clientbound::status::StatusResponse::empty()),
    );

    functions.add(
        Fid::StatusPong,
        Box::new(clientbound::status::StatusPong::empty()),
    );

    // Serverbound
    functions.add(
        Fid::StatusRequest,
        Box::new(serverbound::status::StatusRequest::empty()),
    );

    functions.add(
        Fid::StatusPing,
        Box::new(serverbound::status::StatusPing::empty()),
    );

    // Login
    // Clientbound
    functions.add(
        Fid::Disconnect,
        Box::new(clientbound::login::Disconnect::empty()),
    );

    functions.add(
        Fid::EncRequest,
        Box::new(clientbound::login::EncRequest::empty()),
    );

    functions.add(
        Fid::LoginSuccess,
        Box::new(clientbound::login::LoginSuccess::empty()),
    );

    functions.add(
        Fid::SetCompression,
        Box::new(clientbound::login::SetCompression::empty()),
    );

    functions.add(
        Fid::PluginRequest,
        Box::new(clientbound::login::PluginRequest::empty()),
    );

    // Serverbound
    functions.add(
        Fid::LoginStart,
        Box::new(serverbound::login::LoginStart::empty()),
    );

    functions.add(
        Fid::EncResponse,
        Box::new(serverbound::login::EncResponse::empty()),
    );

    functions.add(
        Fid::PluginResponse,
        Box::new(serverbound::login::PluginResponse::empty()),
    );

    // Play
    // Clientbound
    functions.add(
        Fid::SpawnEntity,
        Box::new(clientbound::play::SpawnEntity::empty()),
    );

    functions.add(
        Fid::SpawnXpOrb,
        Box::new(clientbound::play::SpawnXpOrb::empty()),
    );

    functions.add(
        Fid::SpawnLivingEntity,
        Box::new(clientbound::play::SpawnLivingEntity::empty()),
    );

    functions.add(
        Fid::SpawnPainting,
        Box::new(clientbound::play::SpawnPainting::empty()),
    );

    functions.add(
        Fid::SpawnPlayer,
        Box::new(clientbound::play::SpawnPlayer::empty()),
    );

    functions.add(
        Fid::AckPlayerDigging,
        Box::new(clientbound::play::AckPlayerDigging::empty()),
    );

    functions.add(
        Fid::ChatMessageClientbound,
        Box::new(clientbound::play::ChatMessageClientbound::empty()),
    );

    functions.add(
        Fid::TabCompleteClientbound,
        Box::new(clientbound::play::TabCompleteClientbound::empty()),
    );

    functions.add(
        Fid::ResourcePackSend,
        Box::new(clientbound::play::ResourcePackSend::empty()),
    );

    // Serverbound
    functions.add(
        Fid::ChatMessageServerbound,
        Box::new(serverbound::play::ChatMessageServerbound::empty()),
    );

    functions
}
