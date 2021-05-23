use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub logging_packets: Vec<String>,
}

pub fn get_config() -> Configuration {
    let mut settings = Config::new();

    settings.merge(File::with_name("config")).unwrap();

    settings.try_into().unwrap()
}
