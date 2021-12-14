use serde::Serialize;
use std::{fmt, sync::Arc};

pub type DataQueue = deadqueue::unlimited::Queue<Vec<u8>>;

#[derive(Clone)]
pub struct Queues {
    pub client_proxy: Arc<DataQueue>,
    pub proxy_client: Arc<DataQueue>,
    pub server_proxy: Arc<DataQueue>,
    pub proxy_server: Arc<DataQueue>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Slot {
    pub present: bool,
    pub item_id: Option<i32>,
    pub item_count: Option<i8>,
    pub nbt: Option<nbt::Blob>,
}
