use std::fmt;

pub mod v754;

pub mod clientbound;
pub mod serverbound;

pub(crate) mod macros;
pub(crate) mod parsable;
pub(crate) mod utils;

pub use parsable::Parsable;

pub use v754 as current_protocol;

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
    ChatMessageServerbound,
    // SpawnEntity,
    // SpawnXpOrb,
    // SpawnLivingEntity,
    // SpawnPainting,
    // SpawnPlayer,
    // AckPlayerDigging,
    // ChatMessageClientbound,
    // TabCompleteClientbound,
    // ChatMessageServerbound,
    // ResourcePackSend,
    // ClientSettings,
    // UpdateHealth,
    // PlayerPosition,
    // PlayerPositionRotation,
    // PlayerAbilities,
    // KeepAliveCb,
    // KeepAliveSb,
    // UpdateScore,
    // DisplayScoreboard,
    // ScoreboardObjective,
    // Teams,
    // ResourcePackStatus,
    // EntityEffect,
    // JoinGame,
    // OpenBook,
    // WindowItems,
    // SetPassenger,
    // SteerVehicle,
    // EntityAction,
    // PlayerPositionAndLook,
    // ChunkData,
    // PlayerBlockPlace,
}

impl fmt::Display for Fid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{:?}", self).trim_end_matches(char::is_numeric)
        )
    }
}
