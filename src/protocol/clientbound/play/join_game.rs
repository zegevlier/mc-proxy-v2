use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
    packet::Packet,
    parsable::Parsable,
    raw_packet::RawPacket,
    utils::make_string_fixed_length,
    Direction, SharedState,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct JoinGame {
    pub player_entity_id: i32,
    pub is_hardcore: bool,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub world_count: i32,
    pub world_names: Vec<String>,
    pub dimension_codec: DimentionCodec,
    pub dimension: DimentionType,
    pub world_name: String,
    pub hashed_seed: i64,
    pub max_players: i32,
    pub view_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DimentionCodec {
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type_registry: DimentionTypeRegistry,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome_registry: BiomeRegistry,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DimentionTypeRegistry {
    pub r#type: String,
    pub value: Vec<DimensionTypeRegistryEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DimensionTypeRegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: DimentionType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DimentionType {
    pub piglin_safe: i8,
    pub natural: i8,
    pub ambient_light: f32,
    pub fixed_time: Option<f64>,
    pub infiniburn: String,
    pub respawn_anchor_works: i8,
    pub has_skylight: i8,
    pub bed_works: i8,
    pub effects: String,
    pub has_raids: i8,
    pub logical_height: i32,
    pub coordinate_scale: f32,
    pub ultrawarm: i8,
    pub has_ceiling: i8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BiomeRegistry {
    pub r#type: String,
    pub value: Vec<BiomeRegistryEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BiomeRegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: BiomeProperties,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BiomeProperties {
    pub precipitation: String,
    pub depth: f32,
    pub temperature: f32,
    pub scale: f32,
    pub downfall: f32,
    pub category: String,
    pub temperature_modifier: Option<String>,
    pub effects: Effects,
    pub particle: Option<Particle>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Effects {
    pub sky_color: i32,
    pub water_fog_color: i32,
    pub fog_color: i32,
    pub water_color: i32,
    pub foliage_color: Option<i32>,
    pub grass_color: Option<i32>,
    pub grass_color_modifier: Option<String>,
    pub music: Option<Music>,
    pub ambient_sound: Option<String>,
    pub additions_sound: Option<AdditionsSound>,
    pub mood_sound: Option<MoodSound>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Music {
    pub replace_current_music: i8,
    pub sound: String,
    pub max_delay: i32,
    pub min_delay: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MoodSound {
    pub sound: String,
    pub tick_delay: i32,
    pub offset: f64,
    pub block_search_extent: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Particle {
    pub probability: f32,
    pub options: ParticleOptions,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ParticleOptions {
    pub r#type: String,
}

#[async_trait::async_trait]
impl Parsable for JoinGame {
    fn default() -> Self {
        Self {
            player_entity_id: 0,
            is_hardcore: false,
            gamemode: 0,
            previous_gamemode: 0,
            world_count: 0,
            world_names: Vec::new(),
            dimension_codec: DimentionCodec {
                dimension_type_registry: DimentionTypeRegistry {
                    r#type: String::new(),
                    value: Vec::new(),
                },
                biome_registry: BiomeRegistry {
                    r#type: String::new(),
                    value: Vec::new(),
                },
            },
            dimension: DimentionType {
                piglin_safe: 0,
                natural: 0,
                ambient_light: 0f32,
                fixed_time: None,
                infiniburn: String::new(),
                respawn_anchor_works: 0,
                has_skylight: 0,
                bed_works: 0,
                effects: String::new(),
                has_raids: 0,
                logical_height: 0,
                coordinate_scale: 0f32,
                ultrawarm: 0,
                has_ceiling: 0,
            },
            world_name: String::new(),
            hashed_seed: 0,
            max_players: 0,
            view_distance: 0,
            reduced_debug_info: false,
            enable_respawn_screen: false,
            is_debug: false,
            is_flat: false,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.player_entity_id = packet.decode_int()?;
        self.is_hardcore = packet.decode_bool()?;
        self.gamemode = packet.decode_ubyte()?;
        self.previous_gamemode = packet.decode_byte()?;
        self.world_count = packet.decode_varint()?;
        for _ in 0..self.world_count {
            self.world_names.push(packet.decode_string()?);
        }
        self.dimension_codec = packet.decode_nbt()?;
        self.dimension = packet.decode_nbt()?;
        self.world_name = packet.decode_identifier()?;
        self.hashed_seed = packet.decode_long()?;
        self.max_players = packet.decode_varint()?;
        self.view_distance = packet.decode_varint()?;
        self.reduced_debug_info = packet.decode_bool()?;
        self.enable_respawn_screen = packet.decode_bool()?;
        self.is_debug = packet.decode_bool()?;
        self.is_flat = packet.decode_bool()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {:?} {} {} {} {} {} {} {} {} {} {}",
            self.player_entity_id,
            self.is_hardcore,
            self.gamemode,
            self.previous_gamemode,
            self.world_names,
            make_string_fixed_length(format!("{:?}", self.dimension_codec), 16),
            make_string_fixed_length(format!("{:?}", self.dimension), 16),
            self.world_name,
            self.hashed_seed,
            self.max_players,
            self.view_distance,
            self.reduced_debug_info,
            self.enable_respawn_screen,
            self.is_debug,
            self.is_flat
        )
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        _status: &mut SharedState,
        plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        let mut join_game_packet: JoinGame = self.to_owned();

        for plugin in plugins {
            match plugin.edit_join_game(&join_game_packet) {
                Some(new_join_game) => {
                    join_game_packet = new_join_game;
                }
                None => continue,
            }
        }
        let mut raw_packet = RawPacket::new();
        raw_packet.encode_int(join_game_packet.player_entity_id);
        raw_packet.encode_bool(join_game_packet.is_hardcore);
        raw_packet.encode_ubyte(join_game_packet.gamemode);
        raw_packet.encode_byte(join_game_packet.previous_gamemode);
        raw_packet.encode_varint(join_game_packet.world_names.len() as i32);
        for wn in join_game_packet.world_names {
            raw_packet.encode_string(wn);
        }
        raw_packet.encode_nbt(join_game_packet.dimension_codec);
        raw_packet.encode_nbt(join_game_packet.dimension);
        raw_packet.encode_identifier(join_game_packet.world_name);
        raw_packet.encode_long(join_game_packet.hashed_seed);
        raw_packet.encode_varint(join_game_packet.max_players);
        raw_packet.encode_varint(join_game_packet.view_distance);
        raw_packet.encode_bool(join_game_packet.reduced_debug_info);
        raw_packet.encode_bool(join_game_packet.enable_respawn_screen);
        raw_packet.encode_bool(join_game_packet.is_debug);
        raw_packet.encode_bool(join_game_packet.is_flat);

        Ok(vec![(
            Packet::from(raw_packet, fid_to_pid(Fid::JoinGame)),
            Direction::Clientbound,
        )])
    }
}
