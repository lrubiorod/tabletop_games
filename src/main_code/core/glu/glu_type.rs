use crate::main_code::core::game_state::GameState;
use crate::main_code::core::glu::glu::GluTrait;

/// Enum representing the type of GLU
#[derive(Clone, Debug, PartialEq)]
pub enum GLUType {
    Default,

    // TODO: Remove TypeA example
    TypeA(TypeA),
}

impl Default for GLUType {
    fn default() -> Self {
        GLUType::Default
    }
}

/// Implementation of the GLU trait for GLUType
impl GluTrait for GLUType {
    fn execute(
        &mut self,
        game_state: &mut Box<dyn GameState>,
        player_id: usize,
        parent_id: Option<usize>,
    ) -> bool {
        match self {
            GLUType::TypeA(t) => t.execute(game_state, player_id, parent_id),
            _ => false,
        }
    }

    fn can_execute(&self, game_state: &Box<dyn GameState>, player_id: usize) -> bool {
        match self {
            GLUType::TypeA(t) => t.can_execute(game_state, player_id),
            _ => false,
        }
    }
}

// TODO: Remove TypeA example
#[derive(Clone, Debug, PartialEq)]
pub struct TypeA {}

impl GluTrait for TypeA {
    fn execute(
        &mut self,
        _game_state: &mut Box<dyn GameState>,
        _player_id: usize,
        _parent_id: Option<usize>,
    ) -> bool {
        println!("Executing TypeA GLU");
        true
    }

    fn can_execute(&self, _game_state: &Box<dyn GameState>, _player_id: usize) -> bool {
        true
    }
}
