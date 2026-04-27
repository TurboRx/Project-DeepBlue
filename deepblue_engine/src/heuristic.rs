use crate::state::{BattleState, PlayerState};

pub struct Evaluator {
    // We can store weights here later
    pub hp_weight: f32,
    pub speed_tier_weight: f32,
    pub hazard_weight: f32,
    pub momentum_weight: f32,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self {
            hp_weight: 1.0,
            speed_tier_weight: 0.5,
            hazard_weight: 0.3,
            momentum_weight: 0.4,
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates the battle state from the perspective of Player 1.
    /// Returns a score where positive means P1 is winning, negative means P2 is winning.
    pub fn evaluate(&self, state: &BattleState) -> f32 {
        let p1_score = self.evaluate_player(&state.p1, &state.p2, state);
        let p2_score = self.evaluate_player(&state.p2, &state.p1, state);

        p1_score - p2_score
    }

    fn evaluate_player(&self, player: &PlayerState, opponent: &PlayerState, _battle: &BattleState) -> f32 {
        let mut score = 0.0;

        // 1. HP and Fainted Status
        score += self.evaluate_hp(player) * self.hp_weight;

        // 2. Speed Tier Dominance (Skeleton)
        score += self.evaluate_speed_tiers(player, opponent) * self.speed_tier_weight;

        // 3. Entry Hazard Pressure
        score -= self.evaluate_hazards(player) * self.hazard_weight; // Hazards on my side are bad

        // 4. Positional Momentum (Skeleton)
        score += self.evaluate_momentum(player, opponent) * self.momentum_weight;

        score
    }

    fn evaluate_hp(&self, player: &PlayerState) -> f32 {
        let mut hp_score = 0.0;
        for poke in &player.team {
            if !poke.fainted && poke.max_hp > 0 {
                hp_score += (poke.hp as f32) / (poke.max_hp as f32);
            }
        }
        hp_score
    }

    fn evaluate_speed_tiers(&self, _player: &PlayerState, _opponent: &PlayerState) -> f32 {
        // TODO: Calculate real speed including modifiers (Choice Scarf, tailwind, stat drops)
        // For now, return a dummy value
        0.0
    }

    fn evaluate_hazards(&self, player: &PlayerState) -> f32 {
        let mut hazard_penalty = 0.0;
        let h = player.hazards;

        if h.contains(crate::state::FieldHazards::STEALTH_ROCK) {
            hazard_penalty += 1.0; // In reality, depends on team weakness
        }
        if h.contains(crate::state::FieldHazards::SPIKES_1) {
            hazard_penalty += 0.5;
        }
        if h.contains(crate::state::FieldHazards::SPIKES_2) {
            hazard_penalty += 1.0;
        }
        if h.contains(crate::state::FieldHazards::SPIKES_3) {
            hazard_penalty += 1.5;
        }
        if h.contains(crate::state::FieldHazards::TOXIC_SPIKES_1) {
            hazard_penalty += 0.5;
        }
        if h.contains(crate::state::FieldHazards::TOXIC_SPIKES_2) {
            hazard_penalty += 1.5;
        }
        if h.contains(crate::state::FieldHazards::STICKY_WEB) {
            hazard_penalty += 1.0;
        }

        hazard_penalty
    }

    fn evaluate_momentum(&self, _player: &PlayerState, _opponent: &PlayerState) -> f32 {
        // TODO: Calculate momentum based on free switches, slow u-turns, active matchup advantages.
        0.0
    }
}
