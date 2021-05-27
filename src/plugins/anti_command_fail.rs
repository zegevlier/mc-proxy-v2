use crate::{functions, packet::Packet, plugin, utils::generate_message_packet, Direction};

#[derive(Clone)]
pub struct Acf {}

impl plugin::EventHandler for Acf {
    fn new() -> Self {
        Self {}
    }

    fn on_message(
        &mut self,
        message: &functions::serverbound::play::ChatMessageServerbound,
    ) -> Option<Vec<(Packet, Direction)>> {
        if message.message.starts_with('.') {
            Some(vec![(
                generate_message_packet("Command not found!").unwrap(),
                Direction::Clientbound,
            )])
        } else {
            None
        }
    }
}
