use packet::{Chat, ProtoEnc};

use crate::packet;
use config_loader::Configuration;
use mcore::types::{Direction, SharedState};

packet! {
    ChatMessageServerbound, all, {
        message: Chat
    }
}

#[async_trait::async_trait]
impl Parsable for ChatMessageServerbound {
    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        _status: &mut SharedState,
        plugins: &mut Vec<Box<dyn plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        let mut return_vec = None;
        for plugin in plugins {
            match plugin.on_message(&self.message) {
                Some(plugin_vec) => {
                    return_vec = Some(plugin_vec);
                    break;
                }
                None => continue,
            }
        }
        if return_vec.is_none() {
            return_vec = Some(vec![(
                Self {
                    message: self.message.clone(),
                }
                .encode_packet()?,
                Direction::Serverbound,
            )]);
        }

        Ok(return_vec.unwrap())
    }
}
