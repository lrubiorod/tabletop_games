use crate::main_code::core::actions::action::Action;
use crate::main_code::core::game_state::GameState;
use rand::prelude::ThreadRng;
use rand::Rng;

pub trait Player: dyn_clone::DynClone {
    fn next_action(
        &mut self,
        observation: &dyn GameState,
        actions: &Vec<Box<dyn Action>>,
    ) -> Box<dyn Action>;
}
dyn_clone::clone_trait_object!(Player);
