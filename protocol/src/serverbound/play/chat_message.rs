use packet::{Chat, ProtoEnc, SharedState};

use crate::packet;
use config_loader::Configuration;
use mcore::types::Direction;

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
    ) -> packet::Result<Option<Vec<(Packet, Direction)>>> {
        let mut plugin_response = None;
        for plugin in plugins {
            if let Some(response) = plugin.on_message(&self.message).await {
                plugin_response = Some(response);
                break;
            }
        }
        if plugin_response.is_none() {
            plugin_response = Some(
                plugin::PluginReponse {
                    send_original: true,
                    packets: vec![],
                }
            )
        }

        let mut return_vec = if plugin_response.as_ref().unwrap().send_original {
            vec![(self.clone().encode_packet().unwrap(), Direction::Serverbound)]
        } else {
            vec![]
        };
            
        return_vec.append(&mut plugin_response.unwrap().packets);

        Ok(Some(return_vec))
    }
}
