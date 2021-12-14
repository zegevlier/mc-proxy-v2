use crate::plugin::EventHandler;

mod rainbowify;

pub fn get_plugins() -> Vec<Box<dyn EventHandler + Send>> {
    let plugin_vec: Vec<Box<dyn EventHandler + Send>> = vec![
        Box::new(rainbowify::Rainbowify::new()),
    ];
    plugin_vec
}
