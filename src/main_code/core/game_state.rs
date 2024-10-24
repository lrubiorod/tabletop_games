use crate::main_code::core::{
    core_constants::GameResult, game_parameters::GameParameters, glu::extended_glu::ExtendedGLU,
    interfaces::action_type::ActionType,
};
use std::collections::VecDeque;

use rand::rngs::ThreadRng;

pub trait GameState {
    /// Determines the current player by checking if there are actions in progress
    fn current_player(&self) -> i8;

    /// Returns the queue of actions in progress
    fn actions_in_progress(&mut self) -> &mut VecDeque<ExtendedGLU>;

    /// Returns the current turn owner (in case no actions are in progress)
    fn turn_owner(&self) -> i8;

    /// Returns the number of players in the game
    fn n_players(&self) -> u8;

    /// Returns random source
    fn rnd(&self) -> ThreadRng;

    fn game_status(&self) -> &GameResult;

    fn set_game_status(&mut self, game_status: GameResult);

    fn player_results(&self) -> &Vec<GameResult>;

    fn set_player_result(&mut self, player_result: GameResult, id: usize);

    fn reset(&mut self);

    fn remove_completed_actions_in_progress(&mut self);
    fn has_pending_actions_in_progress(&mut self) -> bool;

    /*
       fn remove_glu_in_progress(&mut self, id: i32) -> bool;

       fn get_glu_in_progress(&self, id: i32) -> Option<&Box<dyn ExtendedSequence>>;
       fn current_action_in_progress(&self) -> Option<&Box<dyn ExtendedSequence>>;
       fn set_action_in_progress(&mut self, action: Box<dyn ExtendedSequence>, parent_id: i32)
           -> bool;

    */
}

/**
 * Represents the state of the game, containing necessary information about the game.
 * This struct is distinct from the Game struct, which also controls the players and other components not present here.
*/
pub struct AbstractGameState {
    n_players: u8,
    game_parameters: Box<dyn GameParameters>,
    game_status: GameResult,
    player_results: Vec<GameResult>,
    turn_owner: i8,
    // Main RNG used for all random number generation in the game
    // TODO: Review later to check if seed is need
    rnd: ThreadRng,

    // RNG used exclusively for redetermination - not seeded, and doesn't affect the game state
    redetermination_rnd: ThreadRng,

    // Vec of Vec to represent available actions for each player
    player_actions_available: Vec<Vec<Box<dyn ActionType>>>,
    actions_in_progress: VecDeque<ExtendedGLU>,
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
            game_status: GameResult::GameOngoing,
            player_results: vec![GameResult::GameOngoing; n_players as usize],
            turn_owner: 0,
            player_actions_available,
            actions_in_progress: VecDeque::new(), // Initializing actions_in_progress
            rnd: rand::thread_rng(),
            redetermination_rnd: rand::thread_rng(),
        }
    }

    // Gets the index of the action in progress by its ID
    fn get_index_of_action_in_progress(&self, id: i32) -> Option<usize> {
        let mut result: Option<usize> = None;
        for (idx, seq) in self.actions_in_progress.iter().enumerate() {
            if seq.get_id() == id {
                if result.is_some() {
                    panic!("Two actions in progress share the same ID!"); // Ensure no duplicate IDs
                }
                result = Some(idx);
            }
        }
        result
    }
}

impl GameState for AbstractGameState {
    fn current_player(&self) -> i8 {
        // TODO: Use proper current_player
        self.turn_owner
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

    fn game_status(&self) -> &GameResult {
        &self.game_status
    }

    fn set_game_status(&mut self, game_status: GameResult) {
        self.game_status = game_status;
    }

    fn player_results(&self) -> &Vec<GameResult> {
        &self.player_results
    }

    fn set_player_result(&mut self, player_result: GameResult, id: usize) {
        self.player_results[id] = player_result;
    }

    fn reset(&mut self) {
        self.game_status = GameResult::GameOngoing;
        self.player_results = vec![GameResult::GameOngoing; self.n_players as usize];
        self.turn_owner = 0;
        self.actions_in_progress.clear();
    }

    // Removes actions from the stack that are marked as completed
    fn remove_completed_actions_in_progress(&mut self) {
        while let Some(action) = self.actions_in_progress.back() {
            if action.execution_completed() {
                self.actions_in_progress.pop_back(); // Remove completed actions
            } else {
                break; // Stop if the action is not complete
            }
        }
    }

    // Checks if there are any pending actions after cleaning up completed ones
    fn has_pending_actions_in_progress(&mut self) -> bool {
        // This remove_completed_actions_in_progress is essential
        // When an ExtendedGLU is completely executed this is marked on the ExtendedGLU
        // However this does not [currently] actively remove the action from the queue on the game state. Whenever we check the actions_in_progress queue, we
        // therefore first have to remove any completed actions (which is what remove_completed_actions_in_progress() does).
        self.remove_completed_actions_in_progress();

        !self.actions_in_progress.is_empty()
    }

    fn actions_in_progress(&mut self) -> &mut VecDeque<ExtendedGLU> {
        &mut self.actions_in_progress
    }
    /*
       fn remove_glu_in_progress(&mut self, id: i32) -> bool {
           if id == -1 {
               return false;
           }

           if let Some(index) = self
               .actions_in_progress
               .iter()
               .position(|seq| seq.id() == id)
           {
               self.actions_in_progress.remove(index);
               return true;
           }

           false
       }

       fn get_glu_in_progress(&self, id: i32) -> Option<&Box<dyn ExtendedSequence>> {
           if id == -1 {
               return None;
           }
           self.actions_in_progress.iter().find(|seq| seq.id() == id)
       }

       // Returns the current action in progress (the last in the stack) or None if there are none
       fn current_action_in_progress(&self) -> Option<&Box<dyn ExtendedSequence>> {
           self.actions_in_progress.back()
       }

       // Adds an action to the stack, after the parent action if a valid parent_id is given
       fn set_action_in_progress(
           &mut self,
           action: Box<dyn ExtendedSequence>,
           parent_id: i32,
       ) -> bool {
           if parent_id == -1 {
               self.actions_in_progress.push_back(action);
               return true;
           }

           if let Some(index) = self.get_index_of_action_in_progress(parent_id) {
               self.actions_in_progress.insert(index + 1, action); // Insert after the parent action
               true
           } else {
               false
           }
       }

    */
}
