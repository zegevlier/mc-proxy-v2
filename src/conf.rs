use config::{Config, File, FileFormat};
use serde::Deserialize;

pub struct Configuration {
    pub logging_packets: Vec<String>,
    pub player_uuid: String,
    pub player_auth_token: String,
    pub print_buffer: usize,
}

#[derive(Deserialize)]
pub struct ReadConfiguration {
    pub logging_packets: Vec<String>,
    pub player_uuid: String,
    pub player_auth_token: String,
}

pub fn get_config() -> Configuration {
    let mut settings = Config::new();

    settings
        .merge(File::new("config", FileFormat::Yaml))
        .unwrap();

    let config: ReadConfiguration = settings.try_into().unwrap();
    Configuration {
        logging_packets: config.logging_packets.clone(),
        player_uuid: config.player_uuid,
        player_auth_token: config.player_auth_token,
        print_buffer: config.logging_packets.iter().fold(0, |acc, x| {
            if x.len() > acc {
                x.len()
            } else {
                acc
            }
        }),
    }
}
