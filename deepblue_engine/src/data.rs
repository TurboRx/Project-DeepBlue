use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct BaseStats {
    pub hp: u16,
    pub atk: u16,
    pub def: u16,
    pub spa: u16,
    pub spd: u16,
    pub spe: u16,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PokemonData {
    pub num: u16,
    pub name: String,
    pub types: Vec<String>,
    pub base_stats: BaseStats,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoveData {
    pub num: u16,
    pub name: String,
    pub base_power: u16,
    pub accuracy: serde_json::Value,
    pub category: String,
    pub r#type: String,
    pub priority: i8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PokemonUsage {
    pub items: HashMap<String, f32>,
    pub moves: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub pokedex: HashMap<String, PokemonData>,
    pub moves: HashMap<String, MoveData>,
    pub usage: HashMap<String, PokemonUsage>,
    pub typechart: HashMap<String, HashMap<String, f32>>,
    pub species_to_id: HashMap<String, u16>,
    pub move_to_id: HashMap<String, u16>,
}

impl GameData {
    pub fn load(data_dir: &str) -> Self {
        let pokedex_path = Path::new(data_dir).join("pokedex.json");
        let moves_path = Path::new(data_dir).join("moves.json");
        let typechart_path = Path::new(data_dir).join("typechart.json");
        let usage_path = Path::new(data_dir).join("usage.json");

        let pokedex_str = fs::read_to_string(&pokedex_path).unwrap_or_else(|_| "{}".to_string());
        let moves_str = fs::read_to_string(&moves_path).unwrap_or_else(|_| "{}".to_string());
        let typechart_str = fs::read_to_string(&typechart_path).unwrap_or_else(|_| "{}".to_string());
        let usage_str = fs::read_to_string(&usage_path).unwrap_or_else(|_| "{}".to_string());

        let pokedex: HashMap<String, PokemonData> = serde_json::from_str(&pokedex_str).unwrap_or_default();
        let moves: HashMap<String, MoveData> = serde_json::from_str(&moves_str).unwrap_or_default();
        let usage: HashMap<String, PokemonUsage> = serde_json::from_str(&usage_str).unwrap_or_default();

        let mut typechart = HashMap::new();
        if let Ok(tc_json) = serde_json::from_str::<HashMap<String, Value>>(&typechart_str) {
            for (type_name, type_data) in tc_json {
                if let Some(damage_taken) = type_data.get("damageTaken").and_then(|d| d.as_object()) {
                    let mut type_matchups = HashMap::new();
                    for (attacking_type, modifier) in damage_taken {
                        if let Some(mod_val) = modifier.as_u64() {
                            let mult;
                            // Showdown format: 0=1x, 1=2x, 2=0.5x, 3=0x
                            match mod_val {
                                1 => mult = 2.0,
                                2 => mult = 0.5,
                                3 => mult = 0.0,
                                _ => mult = 1.0,
                            }
                            type_matchups.insert(attacking_type.clone(), mult);
                        }
                    }
                    typechart.insert(type_name, type_matchups);
                }
            }
        }

        let mut species_to_id = HashMap::new();
        for (key, val) in &pokedex {
            species_to_id.insert(key.clone(), val.num);
        }

        let mut move_to_id = HashMap::new();
        for (key, val) in &moves {
            move_to_id.insert(key.clone(), val.num);
        }

        GameData {
            pokedex,
            moves,
            usage,
            typechart,
            species_to_id,
            move_to_id,
        }
    }
}
