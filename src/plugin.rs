// use crate::Direction;
// functions::clientbound::play::{EntityEffect, JoinGame, PlayerAbilities},
// functions::serverbound::play::ChatMessageServerbound,

use dyn_clone::DynClone;
// use packet::Packet;

pub trait EventHandler: DynClone {
    fn new() -> Self
    where
        Self: Sized + Clone;

    // #[allow(unused_variables)]
    // fn on_message(&mut self, message: &ChatMessageServerbound) -> Option<Vec<(Packet, Direction)>> {
    //     None
    // }

    // #[allow(unused_variables)]
    // fn on_move(&mut self, x: f64, y: f64, z: f64) -> Option<Vec<(Packet, Direction)>> {
    //     None
    // }

    // #[allow(unused_variables)]
    // fn on_player_abilities(
    //     &mut self,
    //     player_abilities: &PlayerAbilities,
    // ) -> Option<Vec<(Packet, Direction)>> {
    //     None
    // }

    // #[allow(unused_variables)]
    // fn on_potion_effect_apply(
    //     &mut self,
    //     effect_info: &EntityEffect,
    // ) -> Option<Vec<(Packet, Direction)>> {
    //     None
    // }

    // #[allow(unused_variables)]
    // fn edit_join_game(&mut self, join_game: &JoinGame) -> Option<JoinGame> {
    //     None
    // }
}
dyn_clone::clone_trait_object!(EventHandler);
