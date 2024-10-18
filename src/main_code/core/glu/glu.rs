use crate::main_code::core::{game_state::GameState, glu::glu_type::GLUType};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Trait representing the base behavior for a GLU (Game Logic Unit)
pub trait GluTrait {
    fn execute(
        &mut self,
        game_state: &mut Box<dyn GameState>,
        player_id: usize,
        parent_id: Option<usize>,
    ) -> bool;
    fn can_execute(&self, game_state: &Box<dyn GameState>, player_id: usize) -> bool;
    // TODO: Añadir execute del propio trait
    /*
        /**
     * Executes and informs parent (if any) if completed.
     *
     * @param gs - current game state
     * @param player - player the GLU should be executed for
     * @return - true if GLU completely executed, false otherwise (e.g. if it's set in progress)
     */
    public boolean execute(AbstractGameState gs, int player, int parentID) {
        if (gs.getCoreGameParameters().recordEventHistory) {
            gs.logEvent(Event.GameEvent.GAME_EVENT, "GLU execute: " + this.getString(gs));
        }
        boolean complete = _execute(gs, player);
        if (complete) {
            if (parentID != -1) {
                IExtendedSequence glu = gs.getGLUInProgress(parentID);
                if (glu != null) {
                    glu._childExecuted(gs, this);
                } else {
                    int a = 0;
                }
            }
        }
        return complete;
    }
     */
}

/// Static atomic counter for generating unique IDs across GLUs
static ID_FOUNTAIN: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Default, Debug)]
pub struct GLU {
    pub id: usize,
    pub glu_type: GLUType,
}

impl GLU {
    pub fn new(glu_type: GLUType) -> Self {
        GLU {
            id: Self::next_id(),
            glu_type,
        }
    }

    /// Generates the next unique ID using an atomic counter
    fn next_id() -> usize {
        ID_FOUNTAIN.fetch_add(1, Ordering::Relaxed)
    }

    fn get_id(&self) -> usize {
        self.id
    }
    fn get_type(&self) -> GLUType {
        self.glu_type.clone()
    }
}

impl GluTrait for GLU {
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
}
