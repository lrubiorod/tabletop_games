use crate::main_code::{
    core::{
        actions::{action::Action, do_nothing::DoNothing},
        game_state::GameState,
        player::Player,
    },
    players::human::action_controller::ActionController,
};

#[derive(Clone)]
pub struct HumanGUIPlayer {
    ac: ActionController,
}

impl HumanGUIPlayer {
    pub fn new(ac: ActionController) -> Self {
        HumanGUIPlayer { ac }
    }
}

impl Player for HumanGUIPlayer {
    fn next_action(
        &mut self,
        _observation: &dyn GameState,
        _actions: &Vec<Box<dyn Action>>,
    ) -> Box<dyn Action> {
        // Try to get the action from the ActionController
        self.ac.get_action().unwrap_or_else(|| {
            // If no action is available, return a default action like DoNothing
            Box::new(DoNothing::new())
        })
    }
}
