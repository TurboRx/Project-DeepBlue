use crate::state::BattleState;
use crate::heuristic::Evaluator;
use std::time::{Duration, Instant};
use rayon::prelude::*;

pub fn run_ismcts(root_state: &BattleState, evaluator: &Evaluator, time_limit: Duration) -> String {
    let start_time = Instant::now();
    let num_threads = rayon::current_num_threads();

    // 1. Thread-local search loop
    let best_moves: Vec<_> = (0..num_threads).into_par_iter().map(|_thread_id| {
        let mut iterations = 0;
        let mut thread_best_move = "struggle";
        let mut thread_best_score = f32::NEG_INFINITY;

        while start_time.elapsed() < time_limit {
            // A. Determinize: "Guess" hidden opponent info using rand
            let mut determinized_state = *root_state;
            determinize_state(&mut determinized_state);

            // B. Run MCTS iteration (Selection, Expansion, Simulation, Backpropagation)
            // SKELETON: Just evaluating the root state and randomly trying "moves" for now.
            let score = evaluator.evaluate(&determinized_state);

            // Dummy move selection logic
            if score > thread_best_score {
                thread_best_score = score;
                thread_best_move = "1"; // e.g., move 1
            }

            iterations += 1;
        }

        // Return thread results
        (thread_best_move, thread_best_score, iterations)
    }).collect();

    // 2. Aggregate results
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

fn determinize_state(state: &mut BattleState) {
    // SKELETON: Randomly assign hidden items/moves based on probabilities.
    // In a real implementation, we sample from distributions provided by the Python relay.

    // Example: For each opponent Pokemon, if an item is unknown, assign Leftovers (dummy).
    for poke in &mut state.p2.team {
        if poke.item_id == 0 {
            // Pick a random likely item
            poke.item_id = 15; // e.g., Leftovers id
        }
    }
}
