#![allow(where_clauses_object_safety)]
use dyn_clone::DynClone;
use packet::{Chat, Packet};

use mcore::types::Direction;

pub struct PluginReponse {
    pub send_original: bool,
    pub packets: Vec<(Packet, Direction)>,
}

// Need to be remade into a from that dynamically loads the plugins
// Probably going to be using https://adventures.michaelfbryan.com/posts/plugins-in-rust/
#[async_trait::async_trait]
pub trait EventHandler: DynClone {
    async fn new() -> Self
    where
        Self: Sized + Clone;

    #[allow(unused_variables, where_clauses_object_safety)]
    async fn on_message(&mut self, message: &Chat) -> Option<PluginReponse> {
        None
    }
}
dyn_clone::clone_trait_object!(EventHandler);
