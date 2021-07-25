use crate::{
    functions::clientbound::play::{EntityEffect, JoinGame, PlayerAbilities},
    functions::serverbound::play::ChatMessageServerbound,
    packet::Packet,
    Direction,
};
use dyn_clone::DynClone;

pub trait EventHandler: DynClone {
    fn new() -> Self
    where
        Self: Sized + Clone;

    fn on_message(
        &mut self,
        _message: &ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        None
    }

    fn on_move(&mut self, _x: f64, _y: f64, _z: f64) -> Option<Vec<(Packet, Direction)>> {
        None
    }

    fn on_player_abilities(
        &mut self,
        _player_abilities: &PlayerAbilities,
    ) -> Option<Vec<(Packet, Direction)>> {
        None
    }

    fn on_potion_effect_apply(
        &mut self,
        _effect_info: &EntityEffect,
    ) -> Option<Vec<(Packet, Direction)>> {
        None
    }

    fn edit_join_game(&mut self, _join_game: &JoinGame) -> Option<JoinGame> {
        None
    }
}
dyn_clone::clone_trait_object!(EventHandler);
