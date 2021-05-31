use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub logging_packets: Vec<String>,
    pub player_uuid: String,
    pub player_auth_token: String,
}

pub fn get_config() -> Configuration {
    let mut settings = Config::new();

    settings
        .merge(File::new("config", FileFormat::Yaml))
        .unwrap();

    settings.try_into().unwrap()
}
