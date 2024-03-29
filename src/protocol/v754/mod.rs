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

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
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
    PlayerAbilities,
    KeepAliveCb,
    KeepAliveSb,
    UpdateScore,
    DisplayScoreboard,
    ScoreboardObjective,
    Teams,
    ResourcePackStatus,
    EntityEffect,
    JoinGame,
    OpenBook,
    WindowItems,
    SetPassenger,
    SteerVehicle,
    EntityAction,
    // PlayerPositionAndLook,
    ChunkData,
    PlayerBlockPlace,
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
        Fid::PlayerAbilities => 0x30,
        Fid::KeepAliveCb => 0x1F,
        Fid::KeepAliveSb => 0x10,
        Fid::UpdateScore => 0x4D,
        Fid::DisplayScoreboard => 0x43,
        Fid::ScoreboardObjective => 0x4A,
        Fid::Teams => 0x4C,
        Fid::ResourcePackStatus => 0x21,
        Fid::EntityEffect => 0x59,
        Fid::JoinGame => 0x24,
        Fid::OpenBook => 0x2C,
        Fid::WindowItems => 0x13,
        Fid::SetPassenger => 0x4B,
        Fid::SteerVehicle => 0x1D,
        Fid::EntityAction => 0x1C,
        Fid::ChunkData => 0x20,
        Fid::PlayerBlockPlace => 0x2E,
        // Fid::PlayerPositionAndLook => 0x38,
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
        let map: HashMap<Direction, HashMap<State, Vec<Fid>>> = hashmap! {
            Direction::Clientbound => hashmap! {
                State::Handshaking => vec! [],
                State::Status => vec! [
                    Fid::StatusResponse,
                    Fid::StatusPong
                ],
                State::Login => vec! [
                    Fid::Disconnect,
                    Fid::EncRequest,
                    Fid::LoginSuccess,
                    Fid::SetCompression,
                    Fid::PluginRequest
                ],
                State::Play => vec! [
                    Fid::SpawnEntity,
                    Fid::SpawnXpOrb,
                    Fid::SpawnLivingEntity,
                    Fid::SpawnPainting,
                    Fid::SpawnPlayer,
                    Fid::AckPlayerDigging,
                    Fid::ChatMessageClientbound,
                    Fid::TabCompleteClientbound,
                    Fid::ResourcePackSend,
                    Fid::UpdateHealth,
                    Fid::PlayerAbilities,
                    Fid::KeepAliveCb,
                    Fid::UpdateScore,
                    Fid::DisplayScoreboard,
                    Fid::ScoreboardObjective,
                    Fid::Teams,
                    Fid::EntityEffect,
                    Fid::JoinGame,
                    Fid::OpenBook,
                    Fid::WindowItems,
                    // Fid::PlayerPositionAndLook,
                    Fid::SetPassenger,
                    Fid::ChunkData,
                ],
            },
            Direction::Serverbound => hashmap! {
                State::Handshaking => vec! [
                    Fid::Handshake,
                ],
                State::Status => vec! [
                    Fid::StatusRequest,
                    Fid::StatusPing,
                ],
                State::Login => vec! [
                    Fid::LoginStart,
                    Fid::EncResponse,
                    Fid::PluginResponse,
                ],
                State::Play => vec! [
                    Fid::ChatMessageServerbound,
                    Fid::ClientSettings,
                    Fid::PlayerPosition,
                    Fid::PlayerPositionRotation,
                    Fid::KeepAliveSb,
                    Fid::ResourcePackStatus,
                    Fid::SteerVehicle,
                    Fid::EntityAction,
                    Fid::PlayerBlockPlace,
                ],
            }
        };
        let map = map
            .iter()
            .map(|(direction, state_fid_vec)| {
                (
                    direction.to_owned(),
                    state_fid_vec
                        .iter()
                        .map(|(state, fid_vec)| {
                            let mut hashmap = HashMap::new();
                            for fid in fid_vec.to_owned() {
                                hashmap.insert(fid_to_pid(fid), fid);
                            }
                            (state.to_owned(), hashmap)
                        })
                        .collect::<HashMap<State, HashMap<i32, Fid>>>(),
                )
            })
            .collect();
        Self {
            map,
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
        Box::new(sb::handshaking::Handshake::default()),
    );

    // Status
    // Clientbound
    functions.add(
        Fid::StatusResponse,
        Box::new(cb::status::StatusResponse::default()),
    );

    functions.add(Fid::StatusPong, Box::new(cb::status::StatusPong::default()));

    // Serverbound
    functions.add(
        Fid::StatusRequest,
        Box::new(sb::status::StatusRequest::default()),
    );

    functions.add(Fid::StatusPing, Box::new(sb::status::StatusPing::default()));

    // Login
    // Clientbound
    functions.add(Fid::Disconnect, Box::new(cb::login::Disconnect::default()));

    functions.add(Fid::EncRequest, Box::new(cb::login::EncRequest::default()));

    functions.add(
        Fid::LoginSuccess,
        Box::new(cb::login::LoginSuccess::default()),
    );

    functions.add(
        Fid::SetCompression,
        Box::new(cb::login::SetCompression::default()),
    );

    functions.add(
        Fid::PluginRequest,
        Box::new(cb::login::PluginRequest::default()),
    );

    // Serverbound
    functions.add(Fid::LoginStart, Box::new(sb::login::LoginStart::default()));

    functions.add(
        Fid::EncResponse,
        Box::new(sb::login::EncResponse::default()),
    );

    functions.add(
        Fid::PluginResponse,
        Box::new(sb::login::PluginResponse::default()),
    );

    // Play
    // Clientbound
    functions.add(Fid::SpawnEntity, Box::new(cb::play::SpawnEntity::default()));

    functions.add(Fid::SpawnXpOrb, Box::new(cb::play::SpawnXpOrb::default()));

    functions.add(
        Fid::SpawnLivingEntity,
        Box::new(cb::play::SpawnLivingEntity::default()),
    );

    functions.add(
        Fid::SpawnPainting,
        Box::new(cb::play::SpawnPainting::default()),
    );

    functions.add(Fid::SpawnPlayer, Box::new(cb::play::SpawnPlayer::default()));

    functions.add(
        Fid::AckPlayerDigging,
        Box::new(cb::play::AckPlayerDigging::default()),
    );

    functions.add(
        Fid::ChatMessageClientbound,
        Box::new(cb::play::ChatMessageClientbound::default()),
    );

    functions.add(
        Fid::TabCompleteClientbound,
        Box::new(cb::play::TabCompleteClientbound::default()),
    );

    functions.add(
        Fid::ResourcePackSend,
        Box::new(cb::play::ResourcePackSend::default()),
    );

    functions.add(
        Fid::UpdateHealth,
        Box::new(cb::play::UpdateHealth::default()),
    );

    functions.add(
        Fid::PlayerAbilities,
        Box::new(cb::play::PlayerAbilities::default()),
    );

    functions.add(Fid::KeepAliveCb, Box::new(cb::play::KeepAliveCb::default()));

    functions.add(Fid::UpdateScore, Box::new(cb::play::UpdateScore::default()));

    functions.add(Fid::OpenBook, Box::new(cb::play::OpenBook::default()));

    functions.add(Fid::WindowItems, Box::new(cb::play::WindowItems::default()));

    functions.add(
        Fid::DisplayScoreboard,
        Box::new(cb::play::DisplayScoreboard::default()),
    );

    functions.add(
        Fid::ScoreboardObjective,
        Box::new(cb::play::ScoreboardObjective::default()),
    );

    functions.add(Fid::Teams, Box::new(cb::play::Teams::default()));

    functions.add(
        Fid::EntityEffect,
        Box::new(cb::play::EntityEffect::default()),
    );

    functions.add(Fid::JoinGame, Box::new(cb::play::JoinGame::default()));

    // functions.add(
    //     Fid::PlayerPositionAndLook,
    //     Box::new(cb::play::PlayerPositionAndLook::empty()),
    // );

    functions.add(
        Fid::SetPassenger,
        Box::new(cb::play::SetPassenger::default()),
    );

    functions.add(Fid::ChunkData, Box::new(cb::play::ChunkData::default()));

    // Serverbound
    functions.add(
        Fid::ChatMessageServerbound,
        Box::new(sb::play::ChatMessageServerbound::default()),
    );

    functions.add(
        Fid::ClientSettings,
        Box::new(sb::play::ClientSettings::default()),
    );

    functions.add(
        Fid::PlayerPosition,
        Box::new(sb::play::PlayerPosition::default()),
    );

    functions.add(
        Fid::PlayerPositionRotation,
        Box::new(sb::play::PlayerPositionRotation::default()),
    );

    functions.add(
        Fid::ResourcePackStatus,
        Box::new(sb::play::ResourcePackStatus::default()),
    );

    functions.add(
        Fid::PlayerBlockPlace,
        Box::new(sb::play::PlayerBlockPlace::default()),
    );

    functions.add(Fid::KeepAliveSb, Box::new(sb::play::KeepAliveSb::default()));

    functions.add(
        Fid::SteerVehicle,
        Box::new(sb::play::SteerVehicle::default()),
    );

    functions.add(
        Fid::EntityAction,
        Box::new(sb::play::EntityAction::default()),
    );

    functions
}
