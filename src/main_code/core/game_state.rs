use crate::main_code::core::{
    game_parameters::GameParameters,
    interfaces::{action_type::ActionType, extended_sequence::ExtendedSequence},
};
use std::collections::VecDeque;

use rand::rngs::ThreadRng;

pub trait GameState {
    /// Determines the current player by checking if there are actions in progress
    fn current_player(&self) -> i8;

    /// Returns the queue of actions in progress (polymorphic with ExtendedSequence)
    fn actions_in_progress(&self) -> &VecDeque<Box<dyn ExtendedSequence>>;

    /// Returns the current turn owner (in case no actions are in progress)
    fn turn_owner(&self) -> i8;

    /// Returns the number of players in the game
    fn n_players(&self) -> u8;

    /// Returns random source
    fn rnd(&self) -> ThreadRng;
}

/**
 * Represents the state of the game, containing necessary information about the game.
 * This struct is distinct from the Game struct, which also controls the players and other components not present here.
*/
pub struct AbstractGameState {
    n_players: u8,
    game_parameters: Box<dyn GameParameters>,
    turn_owner: i8,
    // Main RNG used for all random number generation in the game
    // TODO: Review later to check if seed is need
    rnd: ThreadRng,

    // RNG used exclusively for redetermination - not seeded, and doesn't affect the game state
    redetermination_rnd: ThreadRng,

    // Vec of Vec to represent available actions for each player
    player_actions_available: Vec<Vec<Box<dyn ActionType>>>,
    actions_in_progress: VecDeque<Box<dyn ExtendedSequence>>,
}

impl AbstractGameState {
    /// Constructor for AbstractGameState
    pub fn new(game_parameters: Box<dyn GameParameters>, n_players: u8) -> Self {
        let mut player_actions_available = Vec::with_capacity(n_players as usize);

        for _ in 0..n_players {
            player_actions_available.push(Vec::new());
        }

        AbstractGameState {
            n_players,
            game_parameters,
            turn_owner: 0,
            player_actions_available,
            actions_in_progress: VecDeque::new(), // Initializing actions_in_progress
            rnd: rand::thread_rng(),
            redetermination_rnd: rand::thread_rng(),
        }
    }
}

impl GameState for AbstractGameState {
    fn current_player(&self) -> i8 {
        // TODO: Use proper current_player
        self.turn_owner
    }

    fn actions_in_progress(&self) -> &VecDeque<Box<dyn ExtendedSequence>> {
        &self.actions_in_progress
    }

    fn turn_owner(&self) -> i8 {
        self.turn_owner
    }

    fn n_players(&self) -> u8 {
        self.n_players
    }

    fn rnd(&self) -> ThreadRng {
        self.rnd.clone()
    }
}
