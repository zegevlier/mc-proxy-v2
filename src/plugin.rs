use dyn_clone::DynClone;
use packet::{Packet, Chat};

use crate::{types::Direction};

// Need to be remade into a from that dynamically loads the plugins
// Probably going to be using https://adventures.michaelfbryan.com/posts/plugins-in-rust/
pub trait EventHandler: DynClone {
    fn new() -> Self
    where
        Self: Sized + Clone;

    #[allow(unused_variables)]
    fn on_message(&mut self, message: &Chat) -> Option<Vec<(Packet, Direction)>> {
        None
    }
}
dyn_clone::clone_trait_object!(EventHandler);
