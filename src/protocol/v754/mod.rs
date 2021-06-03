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

    functions.add(
        Fid::PlayerAbilities,
        Box::new(cb::play::PlayerAbilities::empty()),
    );

    functions.add(Fid::KeepAliveCb, Box::new(cb::play::KeepAliveCb::empty()));

    functions.add(Fid::UpdateScore, Box::new(cb::play::UpdateScore::empty()));

    functions.add(
        Fid::DisplayScoreboard,
        Box::new(cb::play::DisplayScoreboard::empty()),
    );

    functions.add(
        Fid::ScoreboardObjective,
        Box::new(cb::play::ScoreboardObjective::empty()),
    );

    functions.add(Fid::Teams, Box::new(cb::play::Teams::empty()));

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

    functions.add(
        Fid::ResourcePackStatus,
        Box::new(sb::play::ResourcePackStatus::empty()),
    );

    functions.add(Fid::KeepAliveSb, Box::new(sb::play::KeepAliveSb::empty()));

    functions
}
