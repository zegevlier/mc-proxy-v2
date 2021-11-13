use std::fmt;

pub mod v754;

pub mod clientbound;
pub mod serverbound;

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
