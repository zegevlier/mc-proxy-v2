#![allow(where_clauses_object_safety)]
use plugin::EventHandler;

// mod rainbowify;
mod grpc;

pub async fn get_plugins() -> Vec<Box<dyn EventHandler + Send>> {
    let plugin_vec: Vec<Box<dyn EventHandler + Send>> = vec![
        Box::new(grpc::Grcp::new().await),
        // Box::new(rainbowify::Rainbowify::new().await),
    ];
    plugin_vec
}
