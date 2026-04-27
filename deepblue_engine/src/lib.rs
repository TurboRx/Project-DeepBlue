#![allow(unsafe_op_in_unsafe_fn)]

pub mod state;
pub mod heuristic;
pub mod search;

use pyo3::prelude::*;
use std::time::Duration;

#[pyclass]
pub struct DeepBlueEngine {
    evaluator: heuristic::Evaluator,
    // Add internal state here if persistent across turns
}

#[pymethods]
impl DeepBlueEngine {
    #[new]
    pub fn new() -> Self {
        DeepBlueEngine {
            evaluator: heuristic::Evaluator::new(),
        }
    }

    pub fn set_state(&mut self, _state_bytes: &[u8]) {
        // Here we will parse raw state bytes or dict into our internal BattleState
    }

    pub fn search(&mut self, time_limit_ms: u64) -> String {
        // Run ISMCTS
        let state = state::BattleState::default();
        let limit = Duration::from_millis(time_limit_ms);
        let best_move = search::run_ismcts(&state, &self.evaluator, limit);
        format!("move {}", best_move)
    }
}

#[pymodule]
fn deepblue_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DeepBlueEngine>()?;
    Ok(())
}
