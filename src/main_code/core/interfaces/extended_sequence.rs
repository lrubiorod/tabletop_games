use crate::main_code::core::{
    actions::action::Action,
    game_state::GameState
};

pub trait ExtendedSequence {
    /// Returns a list of available actions for the current player
    fn compute_available_actions(&self, state: &Box<dyn GameState>) -> Vec<Box<dyn Action>>;

    /// Returns the current player ID who is making a decision
    fn current_player(&self, state: &Box<dyn GameState>) -> i8;

    /// Called after an action has been executed
    fn after_action(&mut self, state: &Box<dyn GameState>, action: &Box<dyn Action>) {}

    /// Called when a child action in the sequence has been executed
    fn child_executed(&mut self, _state: &Box<dyn GameState>, _action: &dyn Spawnable) {}

    /// Checks if the extended sequence has been completed
    fn execution_complete(&mut self, state: &Box<dyn GameState>) -> bool {
        if self.is_execution_complete(state) {
            self.finalize(state);
            true
        } else {
            false
        }
    }

    /// Finalizes the sequence, performing cleanup as necessary
    fn finalize(&mut self, state: &Box<dyn GameState>) {
        // TODO: state.remove_glu_in_progress(self.id());
        if let Some(spawnable) = self.as_spawnable() {
            spawnable.notify_completion(state);
        }
    }

    /// Verifies if the sequence execution is complete
    fn is_execution_complete(&self, _state: &Box<dyn GameState>) -> bool {
        false
    }

    /// Returns a unique ID for the sequence
    fn id(&self) -> i32 {
        -1
    }

    /// Returns a string representation of the sequence
    fn get_string(&self, state: &Box<dyn GameState>) -> String;

    /// Checks if this sequence is spawnable (spawns child actions)
    fn as_spawnable(&self) -> Option<&dyn Spawnable> {
        None
    }
}

pub trait Spawnable {
    fn notify_completion(&self, state: &Box<dyn GameState>);
}