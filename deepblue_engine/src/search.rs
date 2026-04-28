use crate::state::BattleState;
use crate::heuristic::Evaluator;
use crate::data::GameData;
use std::time::{Duration, Instant};
use rayon::prelude::*;
use rand::{Rng, RngExt};

pub fn run_ismcts(root_state: &BattleState, evaluator: &Evaluator, game_data: &GameData, time_limit: Duration) -> String {
    let start_time = Instant::now();
    let num_threads = rayon::current_num_threads();

    let best_moves: Vec<_> = (0..num_threads).into_par_iter().map(|_thread_id| {
        let mut iterations = 0;
        let mut thread_best_move = "struggle";
        let mut thread_best_score = f32::NEG_INFINITY;
        let mut rng = rand::rng();

        while start_time.elapsed() < time_limit {
            let mut determinized_state = *root_state;
            determinize_state(&mut determinized_state, game_data, &mut rng);

            let score = evaluator.evaluate(&determinized_state);

            if score > thread_best_score {
                thread_best_score = score;
                thread_best_move = "1"; // e.g., move 1
            }

            iterations += 1;
        }

        (thread_best_move, thread_best_score, iterations)
    }).collect();

    let mut global_best_move = "struggle";
    let mut global_best_score = f32::NEG_INFINITY;
    let mut total_iterations = 0;

    for (mv, score, iters) in best_moves {
        total_iterations += iters;
        if score > global_best_score {
            global_best_score = score;
            global_best_move = mv;
        }
    }

    println!("ISMCTS completed {} iterations. Best move: {}", total_iterations, global_best_move);
    global_best_move.to_string()
}

fn determinize_state<R: Rng>(state: &mut BattleState, game_data: &GameData, rng: &mut R) {
    for poke in &mut state.p2.team {
        if poke.species_id == 0 { continue; }

        // Find pokemon name to lookup usage stats
        let species_name = game_data.species_to_id.iter()
            .find(|&(_, &id)| id == poke.species_id)
            .map(|(name, _)| name.clone());

        if let Some(name) = species_name {
            if let Some(usage) = game_data.usage.get(&name) {
                // Determine item based on usage probability
                if poke.item_id == 0 {
                    let mut cumulative = 0.0;
                    let rand_val = rng.random_range(0.0..1.0);
                    for (_item_name, &prob) in &usage.items {
                        cumulative += prob;
                        if rand_val <= cumulative {
                            // Dummy item id assignment, real implementation would map item name to id
                            poke.item_id = 15;
                            break;
                        }
                    }
                    if poke.item_id == 0 {
                        poke.item_id = 15; // default fallback Leftovers
                    }
                }

                // Determine unknown moves based on usage probability
                for move_id in &mut poke.moves {
                    if *move_id == 0 {
                        let mut cumulative = 0.0;
                        let rand_val = rng.random_range(0.0..1.0);
                        for (move_name, &prob) in &usage.moves {
                            cumulative += prob;
                            if rand_val <= cumulative {
                                if let Some(&id) = game_data.move_to_id.get(move_name) {
                                    *move_id = id;
                                    break;
                                }
                            }
                        }
                        if *move_id == 0 {
                            *move_id = 33; // default fallback Tackle
                        }
                    }
                }
            } else {
                // Fallback for no usage data
                if poke.item_id == 0 { poke.item_id = 15; }
                for move_id in &mut poke.moves {
                    if *move_id == 0 { *move_id = 33; }
                }
            }
        }
    }
}
