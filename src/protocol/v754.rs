use std::collections::HashMap;

pub use super::{clientbound, serverbound, Fid};

use maplit::hashmap;

use crate::functions_macro;
use crate::{
    parsable::Parsable,
    types::{Direction, State},
};

pub struct Functions {
    map: HashMap<Direction, HashMap<State, HashMap<i32, Fid>>>,
    list: HashMap<Fid, Box<dyn Parsable + Send + Sync>>,
}

functions_macro! {
    clientbound {
        handshaking {
            ,
        }
        status {
            0x00 => StatusResponse,
            0x01 => StatusPong,
        }
        login {
            0x00 => Disconnect,
            0x01 => EncRequest,
            0x02 => LoginSuccess,
            0x03 => SetCompression,
            0x04 => PluginRequest,
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
            0x13 => WindowItems,
            0x1F => KeepAliveCb,
            0x20 => ChunkData,
            0x24 => JoinGame,
            0x2C => OpenBook,
            0x30 => PlayerAbilities,
            0x34 => PlayerPositionAndLook,
            0x38 => ResourcePackSend,
            0x43 => DisplayScoreboard,
            0x49 => UpdateHealth,
            0x4A => ScoreboardObjective,
            0x4B => SetPassenger,
            0x4C => Teams,
            0x4D => UpdateScore,
            0x59 => EntityEffect,
        }
    }
    serverbound {
        handshaking {
            0x00 => Handshake,
        }
        status {
            0x00 => StatusRequest,
            0x01 => StatusPing,
        }
        login {
            0x00 => LoginStart,
            0x01 => EncResponse,
            0x02 => PluginResponse,
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
            0x2E => PlayerBlockPlace,
        }
    }
}
