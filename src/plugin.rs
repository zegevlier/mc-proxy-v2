use crate::{functions::serverbound::play::ChatMessageServerbound, packet::Packet, Direction};
use dyn_clone::DynClone;

pub trait EventHandler: DynClone {
    fn new() -> Self
    where
        Self: Sized;

    fn on_message(
        &mut self,
        _message: &ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        None
    }

    fn on_move(&mut self, _x: f64, _y: f64, _z: f64) -> Option<Vec<(Packet, Direction)>> {
        None
    }
}
