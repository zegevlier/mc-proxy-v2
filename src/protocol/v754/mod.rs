use std::{collections::HashMap, fmt};

pub mod clientbound;
pub mod serverbound;

use maplit::hashmap;

use crate::functions_macro;
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
    PlayerPositionAndLook,
    ChunkData,
    PlayerBlockPlace,
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

functions_macro! {
    clientbound {
        handshaking {
        }
        status {
            0x00 => StatusResponse,
            0x01 => StatusPong
        }
        login {
            0x00 => Disconnect,
            0x01 => EncRequest,
            0x02 => LoginSuccess,
            0x03 => SetCompression,
            0x04 => PluginRequest
        }
        play {
            0x00 => SpawnEntity,
            0x01 => SpawnXpOrb,
            0x02 => SpawnLivingEntity,
            0x03 => SpawnPainting,
            0x04 => SpawnPlayer,
            0x07 => AckPlayerDigging,
            0x0E => ChatMessageClientbound,
            0x0F => TabCompleteClientbound,
            0x38 => ResourcePackSend,
            0x49 => UpdateHealth,
            0x30 => PlayerAbilities,
            0x1F => KeepAliveCb,
            0x4D => UpdateScore,
            0x43 => DisplayScoreboard,
            0x4A => ScoreboardObjective,
            0x4C => Teams,
            0x59 => EntityEffect,
            0x24 => JoinGame,
            0x2C => OpenBook,
            0x13 => WindowItems,
            0x38 => PlayerPositionAndLook,
            0x4B => SetPassenger,
            0x20 => ChunkData
        }
    }
    serverbound {
        handshaking {
            0x00 => Handshake
        }
        status {
            0x00 => StatusRequest,
            0x01 => StatusPing
        }
        login {
            0x00 => LoginStart,
            0x01 => EncResponse,
            0x02 => PluginResponse
        }
        play {
            0x03 => ChatMessageServerbound,
            0x05 => ClientSettings,
            0x12 => PlayerPosition,
            0x13 => PlayerPositionRotation,
            0x10 => KeepAliveSb,
            0x21 => ResourcePackStatus,
            0x1D => SteerVehicle,
            0x1C => EntityAction,
            0x2E => PlayerBlockPlace
        }
    }
}
