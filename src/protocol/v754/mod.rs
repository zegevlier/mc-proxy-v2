use std::{collections::HashMap, fmt};

pub mod clientbound;
pub mod serverbound;

use self::clientbound as cb;
use self::serverbound as sb;

use maplit::hashmap;

use crate::{
    parsable::Parsable,
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
    ClientSettings,
    UpdateHealth,
    PlayerPosition,
    PlayerPositionRotation,
}

pub fn fid_to_pid(fid: Fid) -> i32 {
    match fid {
        Fid::Unparsable => -1,
        Fid::Handshake => 0x00,
        Fid::StatusResponse => 0x00,
        Fid::StatusPong => 0x01,
        Fid::StatusRequest => 0x00,
        Fid::StatusPing => 0x01,
        Fid::Disconnect => 0x00,
        Fid::EncRequest => 0x01,
        Fid::LoginSuccess => 0x02,
        Fid::SetCompression => 0x03,
        Fid::PluginRequest => 0x04,
        Fid::LoginStart => 0x00,
        Fid::EncResponse => 0x01,
        Fid::PluginResponse => 0x02,
        Fid::SpawnEntity => 0x00,
        Fid::SpawnXpOrb => 0x01,
        Fid::SpawnLivingEntity => 0x02,
        Fid::SpawnPainting => 0x03,
        Fid::SpawnPlayer => 0x04,
        Fid::AckPlayerDigging => 0x07,
        Fid::ChatMessageClientbound => 0x0E,
        Fid::TabCompleteClientbound => 0x0F,
        Fid::ChatMessageServerbound => 0x03,
        Fid::ResourcePackSend => 0x38,
        Fid::ClientSettings => 0x05,
        Fid::UpdateHealth => 0x49,
        Fid::PlayerPosition => 0x12,
        Fid::PlayerPositionRotation => 0x13,
    }
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
                        0x49 => Fid::UpdateHealth,
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
                        0x05 => Fid::ClientSettings,
                        0x12 => Fid::PlayerPosition,
                        0x13 => Fid::PlayerPositionRotation,
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
        Box::new(sb::handshaking::Handshake::empty()),
    );

    // Status
    // Clientbound
    functions.add(
        Fid::StatusResponse,
        Box::new(cb::status::StatusResponse::empty()),
    );

    functions.add(Fid::StatusPong, Box::new(cb::status::StatusPong::empty()));

    // Serverbound
    functions.add(
        Fid::StatusRequest,
        Box::new(sb::status::StatusRequest::empty()),
    );

    functions.add(Fid::StatusPing, Box::new(sb::status::StatusPing::empty()));

    // Login
    // Clientbound
    functions.add(Fid::Disconnect, Box::new(cb::login::Disconnect::empty()));

    functions.add(Fid::EncRequest, Box::new(cb::login::EncRequest::empty()));

    functions.add(
        Fid::LoginSuccess,
        Box::new(cb::login::LoginSuccess::empty()),
    );

    functions.add(
        Fid::SetCompression,
        Box::new(cb::login::SetCompression::empty()),
    );

    functions.add(
        Fid::PluginRequest,
        Box::new(cb::login::PluginRequest::empty()),
    );

    // Serverbound
    functions.add(Fid::LoginStart, Box::new(sb::login::LoginStart::empty()));

    functions.add(Fid::EncResponse, Box::new(sb::login::EncResponse::empty()));

    functions.add(
        Fid::PluginResponse,
        Box::new(sb::login::PluginResponse::empty()),
    );

    // Play
    // Clientbound
    functions.add(Fid::SpawnEntity, Box::new(cb::play::SpawnEntity::empty()));

    functions.add(Fid::SpawnXpOrb, Box::new(cb::play::SpawnXpOrb::empty()));

    functions.add(
        Fid::SpawnLivingEntity,
        Box::new(cb::play::SpawnLivingEntity::empty()),
    );

    functions.add(
        Fid::SpawnPainting,
        Box::new(cb::play::SpawnPainting::empty()),
    );

    functions.add(Fid::SpawnPlayer, Box::new(cb::play::SpawnPlayer::empty()));

    functions.add(
        Fid::AckPlayerDigging,
        Box::new(cb::play::AckPlayerDigging::empty()),
    );

    functions.add(
        Fid::ChatMessageClientbound,
        Box::new(cb::play::ChatMessageClientbound::empty()),
    );

    functions.add(
        Fid::TabCompleteClientbound,
        Box::new(cb::play::TabCompleteClientbound::empty()),
    );

    functions.add(
        Fid::ResourcePackSend,
        Box::new(cb::play::ResourcePackSend::empty()),
    );

    functions.add(Fid::UpdateHealth, Box::new(cb::play::UpdateHealth::empty()));
    // Serverbound
    functions.add(
        Fid::ChatMessageServerbound,
        Box::new(sb::play::ChatMessageServerbound::empty()),
    );

    functions.add(
        Fid::ClientSettings,
        Box::new(sb::play::ClientSettings::empty()),
    );

    functions.add(
        Fid::PlayerPosition,
        Box::new(sb::play::PlayerPosition::empty()),
    );

    functions.add(
        Fid::PlayerPositionRotation,
        Box::new(sb::play::PlayerPositionRotation::empty()),
    );

    functions
}
