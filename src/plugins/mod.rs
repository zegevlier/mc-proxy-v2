use crate::plugin::EventHandler;
mod anti_command_fail;
mod fake_fly;
mod gamemode;
mod jumpboostcommand;
mod jumptrain;
mod rainbowify;
mod swimspeedcommand;
mod swimtrain;
mod test_plugin;
mod update_game;
mod velocity_monitor;
mod weird_sky;

pub fn get_plugins() -> Vec<Box<dyn EventHandler + Send>> {
    let plugin_vec: Vec<Box<dyn EventHandler + Send>> = vec![
        Box::new(test_plugin::TestPlugin::new()),
        Box::new(rainbowify::Rainbowify::new()),
        Box::new(weird_sky::WeirdSky::new()),
        Box::new(gamemode::Gamemode::new()),
        Box::new(update_game::UpdateGame::new()),
        Box::new(jumptrain::JumpTrain::new()),
        Box::new(swimtrain::SwimTrain::new()),
        Box::new(jumpboostcommand::JumpBoostCommand::new()),
        Box::new(swimspeedcommand::SwimSpeedCommand::new()),
        Box::new(anti_command_fail::Acf::new()),
        // Box::new(velocity_monitor::Velocity::new()),
        // Box::new(fake_fly::FakeFly::new()),
    ];
    plugin_vec
}
