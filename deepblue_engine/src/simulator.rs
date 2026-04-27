use crate::state::{BattleState, Action};
use crate::data::GameData;

pub fn step(state: &mut BattleState, p1_action: Action, p2_action: Action, game_data: &GameData) {
    let p1_speed = calculate_speed(state, true, game_data);
    let p2_speed = calculate_speed(state, false, game_data);

    let (first_player, first_action, second_player, second_action) = if p1_speed >= p2_speed {
        (true, p1_action, false, p2_action)
    } else {
        (false, p2_action, true, p1_action)
    };

    execute_action(state, first_player, first_action, game_data);
    if !is_fainted(state, second_player) {
        execute_action(state, second_player, second_action, game_data);
    }
}

fn calculate_speed(state: &BattleState, is_p1: bool, game_data: &GameData) -> u16 {
    let p_state = if is_p1 { &state.p1 } else { &state.p2 };
    let active_pokemon = &p_state.team[p_state.active_pokemon_index as usize];

    let species_name = game_data.species_to_id.iter().find(|&(_, &id)| id == active_pokemon.species_id).map(|(name, _)| name);
    if let Some(name) = species_name {
        if let Some(p_data) = game_data.pokedex.get(name) {
            let base = p_data.base_stats.spe as f32;
            let speed = ((2.0 * base + 31.0 + (85.0 / 4.0)) * 100.0 / 100.0) + 5.0;
            return speed as u16;
        }
    }
    100
}

fn is_fainted(state: &BattleState, is_p1: bool) -> bool {
    let p_state = if is_p1 { &state.p1 } else { &state.p2 };
    let active_pokemon = &p_state.team[p_state.active_pokemon_index as usize];
    active_pokemon.fainted
}

fn execute_action(state: &mut BattleState, is_p1: bool, action: Action, game_data: &GameData) {
    match action {
        Action::Move(move_idx) => {
            execute_move(state, is_p1, move_idx, game_data);
        }
        Action::Switch(switch_idx) => {
            execute_switch(state, is_p1, switch_idx);
        }
        Action::None => {}
    }
}

fn execute_switch(state: &mut BattleState, is_p1: bool, switch_idx: u8) {
    let p_state = if is_p1 { &mut state.p1 } else { &mut state.p2 };
    p_state.active_pokemon_index = switch_idx;
}

fn execute_move(state: &mut BattleState, is_p1: bool, move_idx: u8, game_data: &GameData) {
    let p_state = if is_p1 { &state.p1 } else { &state.p2 };
    let opp_state = if is_p1 { &state.p2 } else { &state.p1 };

    let attacker = &p_state.team[p_state.active_pokemon_index as usize];
    let defender = &opp_state.team[opp_state.active_pokemon_index as usize];

    if move_idx >= 4 { return; }
    let move_id = attacker.moves[move_idx as usize];
    if move_id == 0 { return; }

    let move_name = game_data.move_to_id.iter().find(|&(_, &id)| id == move_id).map(|(name, _)| name);

    if let Some(name) = move_name {
        if let Some(move_data) = game_data.moves.get(name) {
            let damage = calculate_damage(attacker.species_id, defender.species_id, move_data, game_data);

            let opp_state_mut = if is_p1 { &mut state.p2 } else { &mut state.p1 };
            let defender_mut = &mut opp_state_mut.team[opp_state_mut.active_pokemon_index as usize];

            if damage >= defender_mut.hp {
                defender_mut.hp = 0;
                defender_mut.fainted = true;
            } else {
                defender_mut.hp -= damage;
            }
        }
    }
}

fn calculate_damage(atk_species: u16, def_species: u16, move_data: &crate::data::MoveData, game_data: &GameData) -> u16 {
    let level = 100.0;

    let mut atk_stat = 100.0;
    let mut def_stat = 100.0;
    let mut modifier = 1.0;
    let base_power = move_data.base_power as f32;

    let atk_name = game_data.species_to_id.iter().find(|&(_, &id)| id == atk_species).map(|(name, _)| name);
    let def_name = game_data.species_to_id.iter().find(|&(_, &id)| id == def_species).map(|(name, _)| name);

    if let Some(name) = atk_name {
        if let Some(p_data) = game_data.pokedex.get(name) {
            // STAB
            if p_data.types.contains(&move_data.r#type) {
                modifier *= 1.5;
            }

            // Physical vs Special split
            if move_data.category == "Physical" {
                atk_stat = ((2.0 * p_data.base_stats.atk as f32 + 31.0 + 63.0) * level / 100.0) + 5.0;
            } else if move_data.category == "Special" {
                atk_stat = ((2.0 * p_data.base_stats.spa as f32 + 31.0 + 63.0) * level / 100.0) + 5.0;
            }
        }
    }

    if let Some(name) = def_name {
        if let Some(p_data) = game_data.pokedex.get(name) {
            // Physical vs Special split for Defense
            if move_data.category == "Physical" {
                def_stat = ((2.0 * p_data.base_stats.def as f32 + 31.0 + 63.0) * level / 100.0) + 5.0;
            } else if move_data.category == "Special" {
                def_stat = ((2.0 * p_data.base_stats.spd as f32 + 31.0 + 63.0) * level / 100.0) + 5.0;
            }

            // Type Effectiveness
            for def_type in &p_data.types {
                if let Some(type_data) = game_data.typechart.get(def_type) {
                    if let Some(&mult) = type_data.get(&move_data.r#type) {
                        modifier *= mult;
                    }
                }
            }
        }
    }

    if modifier == 0.0 {
        return 0; // Immune
    }

    let damage = ((2.0 * level / 5.0 + 2.0) * base_power * (atk_stat / def_stat)) / 50.0 + 2.0;

    (damage * modifier) as u16
}
