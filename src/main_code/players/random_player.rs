use crate::main_code::core::{actions::action::Action, game_state::GameState, player::Player};
use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Clone)]
pub struct RandomPlayer {
    rnd: ThreadRng,
}

impl RandomPlayer {
    pub fn new() -> Self {
        RandomPlayer {
            rnd: rand::thread_rng(),
        }
    }
}
impl Player for RandomPlayer {
    fn next_action(
        &mut self,
        _observation: &dyn GameState,
        actions: &Vec<Box<dyn Action>>,
    ) -> Box<dyn Action> {
        if actions.is_empty() {
            panic!("No actions available");
        }

        let random_index = self.rnd.gen_range(0..actions.len());
        actions[random_index].clone()
    }
}
