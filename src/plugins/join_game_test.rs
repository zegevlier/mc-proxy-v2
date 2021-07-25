use crate::{functions::clientbound::play::JoinGame, plugin};

#[derive(Clone)]
pub struct JoinGameTest {}

impl plugin::EventHandler for JoinGameTest {
    fn new() -> Self {
        Self {}
    }

    fn edit_join_game(&mut self, _join_game: &JoinGame) -> Option<JoinGame> {
        let mut join_game: JoinGame = _join_game.to_owned();
        for biome_entry in join_game.dimension_codec.biome_registry.value.iter_mut() {
            let current_colour = 0xffffff;
            biome_entry.element.effects.sky_color = current_colour;
            biome_entry.element.effects.water_color = current_colour;
            biome_entry.element.effects.fog_color = current_colour;
            biome_entry.element.effects.water_fog_color = current_colour;
            biome_entry.element.effects.foliage_color = Some(current_colour);
            biome_entry.element.precipitation = "none".to_string();
        }
        Some(join_game)
    }
}
