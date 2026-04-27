#![allow(unsafe_op_in_unsafe_fn)]

pub mod state;
pub mod data;
pub mod heuristic;
pub mod search;
pub mod simulator;

use pyo3::prelude::*;
use std::time::Duration;
use crate::data::GameData;
use crate::state::{BattleState, PlayerState, PokemonState};
use serde_json::Value;

#[pyclass]
pub struct DeepBlueEngine {
    evaluator: heuristic::Evaluator,
    game_data: GameData,
    current_state: BattleState,
}

#[pymethods]
impl DeepBlueEngine {
    #[new]
    #[pyo3(signature = (data_dir="data"))]
    pub fn new(data_dir: &str) -> Self {
        DeepBlueEngine {
            evaluator: heuristic::Evaluator::new(),
            game_data: GameData::load(data_dir),
            current_state: BattleState::default(),
        }
    }

    pub fn set_state(&mut self, state_json: &str) {
        match serde_json::from_str::<Value>(state_json) {
            Ok(val) => {
                let mut state = BattleState::default();

                if let Some(turn) = val.get("turn").and_then(|t| t.as_u64()) {
                    state.turn = turn as u16;
                }

                if let Some(p1) = val.get("p1") {
                    self.parse_player_state(p1, &mut state.p1);
                }
                if let Some(p2) = val.get("p2") {
                    self.parse_player_state(p2, &mut state.p2);
                }

                self.current_state = state;
            },
            Err(e) => {
                println!("[Rust] Error parsing state JSON: {}", e);
            }
        }
    }

    pub fn search(&mut self, time_limit_ms: u64) -> String {
        let limit = Duration::from_millis(time_limit_ms);
        let best_move = search::run_ismcts(&self.current_state, &self.evaluator, &self.game_data, limit);
        format!("move {}", best_move)
    }
}

impl DeepBlueEngine {
    fn parse_player_state(&self, json: &Value, player: &mut PlayerState) {
        if let Some(team) = json.get("team").and_then(|t| t.as_array()) {
            for (i, poke_json) in team.iter().enumerate().take(6) {
                let mut poke = PokemonState::default();

                if let Some(name) = poke_json.get("name").and_then(|n| n.as_str()) {
                    let key = name.to_lowercase().replace(" ", "");
                    if let Some(&id) = self.game_data.species_to_id.get(&key) {
                        poke.species_id = id;
                    }
                }

                poke.hp = poke_json.get("hp").and_then(|h| h.as_u64()).unwrap_or(0) as u16;
                poke.max_hp = poke_json.get("max_hp").and_then(|h| h.as_u64()).unwrap_or(100) as u16;
                poke.active = poke_json.get("active").and_then(|a| a.as_bool()).unwrap_or(false);
                poke.fainted = poke_json.get("fainted").and_then(|f| f.as_bool()).unwrap_or(true);

                if poke.active {
                    player.active_pokemon_index = i as u8;
                }

                if let Some(moves) = poke_json.get("moves").and_then(|m| m.as_array()) {
                    for (j, m) in moves.iter().enumerate().take(4) {
                        if let Some(m_str) = m.as_str() {
                            let key = m_str.to_lowercase().replace(" ", "");
                            if let Some(&id) = self.game_data.move_to_id.get(&key) {
                                poke.moves[j] = id;
                            }
                        }
                    }
                }

                player.team[i] = poke;
            }
        }
    }
}

#[pymodule]
fn deepblue_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DeepBlueEngine>()?;
    Ok(())
}
