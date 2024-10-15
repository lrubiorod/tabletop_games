use crate::main_code::core::game_state::GameState;
use std::{collections::HashSet, hash::Hash};

pub trait Action: dyn_clone::DynClone + downcast_rs::Downcast {
    /// Executes this action, applying its effect to the given game state.
    fn execute(&self, _gs: &mut Box<dyn GameState>) -> bool {
        false
    }

    /// Returns the string representation of this action.
    fn get_string(&self, gs: &Box<dyn GameState>) -> String;

    /// Returns the string representation of this action from the perspective of a specific player.
    fn get_string_perspective(&self, gs: &Box<dyn GameState>, _perspective_player: i8) -> String {
        // TODO: Show a string with perspective
        self.get_string(gs)
    }

    /// Returns the string representation of this action considering a set of perspective players.
    fn get_string_perspectives(
        &self,
        gs: &Box<dyn GameState>,
        perspective_set: &HashSet<i8>,
    ) -> String {
        let current_player = gs.current_player();
        let perspective = if perspective_set.contains(&current_player) {
            current_player
        } else {
            *perspective_set.iter().next().unwrap_or(&current_player)
        };
        self.get_string_perspective(gs, perspective)
    }

    /// Returns a tooltip for the GUI representation of the action.
    fn get_tooltip(&self, _gs: &Box<dyn GameState>) -> String {
        String::from("")
    }

    /// Returns the ID of the action.
    fn id(&self) -> i32;
}
dyn_clone::clone_trait_object!(Action);
downcast_rs::impl_downcast!(Action);

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct AbstractAction {
    id: i32,
}

impl AbstractAction {
    pub fn new() -> Self {
        Self {
            // TODO: use ExtendedGLU
            id: 0,
        }
    }

    pub fn with_id(id: i32) -> Self {
        Self { id }
    }
}

impl Action for AbstractAction {
    fn get_string(&self, _gs: &Box<dyn GameState>) -> String {
        format!("Action with ID: {}", self.id)
    }

    fn id(&self) -> i32 {
        self.id
    }
}
