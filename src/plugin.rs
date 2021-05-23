use crate::{functions::serverbound::play::ChatMessageServerbound, packet::Packet, Direction};
use dyn_clone::DynClone;

pub trait EventHandler: DynClone {
    fn new() -> Self
    where
        Self: Sized;

    fn on_message(&self, _message: &ChatMessageServerbound) -> Option<Vec<(Packet, Direction)>> {
        None
    }
}
