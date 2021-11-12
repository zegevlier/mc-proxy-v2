use std::fmt::Display;

use crate::{conf::Configuration, plugin::EventHandler, Ciphers, Direction, SharedState};

use packet::Packet;

use async_trait::async_trait;
use dyn_clone::DynClone;
use erased_serde::serialize_trait_object;

#[async_trait]
pub trait Parsable:
    erased_serde::Serialize
    + DynClone
    + Display
    + packet::SafeDefault
    + packet::ProtoDec
    + packet::ProtoEnc
{
    #[allow(unused_variables)]
    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        Ok(())
    }

    fn packet_editing(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    async fn edit_packet(
        &self,
        status: &mut SharedState,
        plugins: &mut Vec<Box<dyn EventHandler + Send>>,
        config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn post_send_update(&self, ciphers: &mut Ciphers, status: &SharedState) -> Result<(), ()> {
        Ok(())
    }
}

dyn_clone::clone_trait_object!(Parsable);
serialize_trait_object!(Parsable);
