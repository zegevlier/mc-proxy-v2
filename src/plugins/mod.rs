use crate::plugin::EventHandler;
mod rainbowify;
mod test_plugin;

pub fn get_plugins() -> Vec<Box<dyn EventHandler + Send>> {
    let plugin_vec: Vec<Box<dyn EventHandler + Send>> = vec![
        Box::new(test_plugin::TestPlugin::new()),
        Box::new(rainbowify::Rainbowify::new()),
    ];
    plugin_vec
}
