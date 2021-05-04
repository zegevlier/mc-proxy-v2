use crate::{packet::Packet, Direction, SharedState};
use async_trait::async_trait;
use dyn_clone::DynClone;

#[async_trait]
pub trait Parsable: DynClone {
    fn empty() -> Self
    where
        Self: Sized;

    fn parse_packet(&mut self, packet: Packet) -> Result<(), ()>;

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
    ) -> Result<(Packet, Direction, SharedState), ()> {
        Ok((Packet::new(), Direction::Clientbound, status))
    }

    fn post_send_updating(&self) -> bool {
        false
    }

    fn post_send_update(&self, _status: &mut SharedState) -> Result<(), ()> {
        Ok(())
    }
}

dyn_clone::clone_trait_object!(Parsable);
