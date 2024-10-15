use crate::main_code::core::game_state::GameState;

pub trait ActionType {
    fn name(&self) -> &str;
    fn parse_string(&self, value: &str) -> Box<dyn ActionType>;
    fn get_default(&self) -> Box<dyn ActionType>;
    fn can_execute(&self, game_state: &Box<dyn GameState>, player: usize) -> bool;
}