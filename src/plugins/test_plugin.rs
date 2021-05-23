use crate::{
    functions::serverbound::play::ChatMessageServerbound, packet::Packet, plugin,
    utils::generate_message_packet, Direction,
};

#[derive(Clone)]
pub struct TestPlugin {}

impl plugin::EventHandler for TestPlugin {
    fn new() -> TestPlugin {
        TestPlugin {}
    }

    fn on_message(&mut self, message: &ChatMessageServerbound) -> Option<Vec<(Packet, Direction)>> {
        if message.message == ".test" {
            return Some(vec![(
                generate_message_packet("Test packet received!").unwrap(),
                Direction::Clientbound,
            )]);
        }
        None
    }
}
