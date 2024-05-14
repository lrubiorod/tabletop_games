use std::{
    hash::{Hash, Hasher},
    time::{SystemTime, UNIX_EPOCH},
};

pub trait GameParameters {}

pub struct AbstractParameters {
    random_seed: u64,
    max_rounds: i32,
    timeout_rounds: i32,
    thinking_time_mins: i64,
    increment_action_s: i64,
    increment_turn_s: i64,
    increment_round_s: i64,
    increment_milestone_s: i64,
}

impl AbstractParameters {
    pub fn new() -> Self {
        AbstractParameters::default()
    }

    fn generate_random_seed() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    pub fn random_seed(&self) -> u64 {
        self.random_seed
    }
    pub fn max_rounds(&self) -> i32 {
        self.max_rounds
    }
    pub fn timeout_rounds(&self) -> i32 {
        self.timeout_rounds
    }
    pub fn thinking_time_mins(&self) -> i64 {
        self.thinking_time_mins
    }
    pub fn increment_action_s(&self) -> i64 {
        self.increment_action_s
    }
    pub fn increment_turn_s(&self) -> i64 {
        self.increment_turn_s
    }
    pub fn increment_round_s(&self) -> i64 {
        self.increment_round_s
    }
    pub fn increment_milestone_s(&self) -> i64 {
        self.increment_milestone_s
    }
}

impl Default for AbstractParameters {
    fn default() -> Self {
        Self {
            random_seed: Self::generate_random_seed(),
            max_rounds: -1,
            timeout_rounds: -1,
            thinking_time_mins: 90,
            increment_action_s: 0,
            increment_turn_s: 0,
            increment_round_s: 0,
            increment_milestone_s: 0,
        }
    }
}

impl GameParameters for AbstractParameters {}

/**
 * Clone this game parameter object.
 *
 * @return - new object with the same parameters, but a new random seed.
 */
impl Clone for AbstractParameters {
    fn clone(&self) -> Self {
        Self {
            random_seed: Self::generate_random_seed(),
            max_rounds: self.max_rounds,
            timeout_rounds: self.timeout_rounds,
            thinking_time_mins: self.thinking_time_mins,
            increment_action_s: self.increment_action_s,
            increment_turn_s: self.increment_turn_s,
            increment_round_s: self.increment_round_s,
            increment_milestone_s: self.increment_milestone_s,
        }
    }
}

impl PartialEq for AbstractParameters {
    fn eq(&self, other: &Self) -> bool {
        self.max_rounds == other.max_rounds
            && self.timeout_rounds == other.timeout_rounds
            && self.thinking_time_mins == other.thinking_time_mins
            && self.increment_action_s == other.increment_action_s
            && self.increment_turn_s == other.increment_turn_s
            && self.increment_round_s == other.increment_round_s
            && self.increment_milestone_s == other.increment_milestone_s
    }
}

impl Eq for AbstractParameters {}

impl Hash for AbstractParameters {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.max_rounds.hash(state);
        self.timeout_rounds.hash(state);
        self.thinking_time_mins.hash(state);
        self.increment_action_s.hash(state);
        self.increment_turn_s.hash(state);
        self.increment_round_s.hash(state);
        self.increment_milestone_s.hash(state);
    }
}
