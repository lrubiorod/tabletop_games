use crate::main_code::core::{
    game_state::GameState,
    glu::{glu::GLU, glu_type::ExtendedGLUType},
};

#[derive(Clone)]
pub struct ExtendedGLU {
    pub id: i32,
    pub glu_type: ExtendedGLUType,
    pub parent_id: i32,
    pub player_id: i8,
    pub complete: bool,
}

impl ExtendedGLU {
    pub fn new(glu_type: ExtendedGLUType) -> Self {
        ExtendedGLU {
            id: GLU::next_id(),
            glu_type,
            parent_id: -1,
            player_id: -1,
            complete: false,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_type(&self) -> ExtendedGLUType {
        self.glu_type.clone()
    }
}

pub trait ExtendedGluTrait {
    fn execute(
        &mut self,
        game_state: &mut Box<dyn GameState>,
        player_id: usize,
        parent_id: Option<usize>,
    ) -> bool;
    fn can_execute(&self, game_state: &Box<dyn GameState>, player_id: usize) -> bool;

    fn current_player(&self, state: &Box<dyn GameState>) -> i8;
}

impl ExtendedGluTrait for ExtendedGLU {
    fn execute(
        &mut self,
        game_state: &mut Box<dyn GameState>,
        player_id: usize,
        parent_id: Option<usize>,
    ) -> bool {
        self.glu_type.execute(game_state, player_id, parent_id)
    }

    fn can_execute(&self, game_state: &Box<dyn GameState>, player_id: usize) -> bool {
        self.glu_type.can_execute(game_state, player_id)
    }

    fn current_player(&self, _state: &Box<dyn GameState>) -> i8 {
        self.player_id
    }
}
