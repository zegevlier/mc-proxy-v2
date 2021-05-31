use crate::{
    conf::Configuration, packet::Packet, plugin::EventHandler, raw_packet::RawPacket, Ciphers,
    Direction, SharedState,
};
use async_trait::async_trait;
use dyn_clone::DynClone;

#[async_trait]
pub trait Parsable: DynClone {
    fn empty() -> Self
    where
        Self: Sized;

    fn parse_packet(&mut self, packet: RawPacket) -> Result<(), ()>;

    fn get_printable(&self) -> String;

    fn update_status(&self, _status: &mut SharedState) -> Result<(), ()> {
        Ok(())
    }

    fn status_updating(&self) -> bool {
        false
    }

    fn packet_editing(&self) -> bool {
        false
    }

    async fn edit_packet(
        &self,
        status: SharedState,
        _plugins: &mut Vec<Box<dyn EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
        Ok((Vec::new(), status))
    }

    fn post_send_updating(&self) -> bool {
        false
    }

    fn post_send_update(&self, _ciphers: &mut Ciphers, _status: &SharedState) -> Result<(), ()> {
        Ok(())
    }
}

dyn_clone::clone_trait_object!(Parsable);
