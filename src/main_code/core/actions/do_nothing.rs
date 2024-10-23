use crate::main_code::core::{
    actions::action::{AbstractAction, Action},
    game_state::GameState,
};

#[derive(Clone)]
pub struct DoNothing {
    data: AbstractAction,
}

impl DoNothing {
    pub fn new() -> Self {
        DoNothing {
            data: AbstractAction::new(),
        }
    }
}

impl Action for DoNothing {
    fn execute(&self, _gs: &mut Box<dyn GameState>) -> bool {
        // This action does nothing, just return true
        true
    }

    fn get_string(&self, _gs: &dyn GameState) -> String {
        "DoNothing".to_string()
    }

    fn id(&self) -> i32 {
        self.data.id()
    }
}
