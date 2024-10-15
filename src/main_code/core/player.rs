use crate::main_code::core::{actions::action::Action, game_state::GameState};

pub trait Player: dyn_clone::DynClone {
    fn next_action(
        &mut self,
        observation: &dyn GameState,
        actions: &Vec<Box<dyn Action>>,
    ) -> Box<dyn Action>;
}
dyn_clone::clone_trait_object!(Player);
