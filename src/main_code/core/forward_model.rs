use crate::main_code::core::{
    actions::action::Action, core_constants::GameResult, game_state::GameState,
    glu::extended_glu::ExtendedGluTrait,
};

pub trait ForwardModel {
    /// Combines both the base and specific setup methods. Called from the game loop.
    ///
    /// # Parameters
    /// - `first_state`: The initial game state.
    fn abstract_setup(&self, first_state: &mut Box<dyn GameState>) {
        first_state.set_game_status(GameResult::GameOngoing);

        for i in 0..first_state.n_players() {
            first_state.set_player_result(GameResult::GameOngoing, i.into());
        }

        // firstState.gamePhase = CoreConstants.DefaultGamePhase.Main;

        self.setup_impl(first_state);

        // firstState.addAllComponents();
    }

    /// Performs the initial game setup according to the game rules. Sets up decks, shuffles, deals
    /// player cards, places tokens on boards, etc.
    ///
    /// # Parameters
    /// - `first_state`: The game state to be modified to the initial game state.
    fn setup_impl(&self, first_state: &mut Box<dyn GameState>);

    fn setup(&self, game_state: &mut Box<dyn GameState>) {
        game_state.reset();
        self.abstract_setup(game_state);
    }

    /// Applies the given action to the game state and executes any other game rules.
    ///
    /// # Parameters
    /// - `current_state`: The current game state, to be modified by the action.
    /// - `action`: The action requested to be played by a player.
    fn next(&self, current_state: &mut Box<dyn GameState>, action: Box<dyn Action>) {
        // TODO: record actions
        //let player = current_state.current_player();
        //current_state.recordAction(action, player);

        self.next_impl(current_state, action);

        current_state.remove_completed_actions_in_progress();

        //currentState.advanceGameTick();
    }

    /// Applies the given action to the game state and executes any other game rules.
    /// Steps:
    /// - Execute player action
    /// - Execute applicable game rules
    /// - Check game over conditions and update the game status and player results accordingly
    /// - Move to the next player if applicable
    ///
    /// # Parameters
    /// - `current_state`: The current game state, to be modified by the action.
    /// - `action`: The action requested to be played by a player.
    fn next_impl(&self, current_state: &mut Box<dyn GameState>, action: Box<dyn Action>);

    fn compute_available_actions(
        &self,
        current_state: &mut Box<dyn GameState>,
    ) -> Vec<Box<dyn Action>> {
        if current_state.has_pending_actions_in_progress() {
            let action = current_state.actions_in_progress().back().unwrap().clone();

            action.compute_available_actions(current_state)
        } else {
            self.compute_available_actions_impl(current_state)
        }
    }

    fn compute_available_actions_impl(
        &self,
        current_state: &Box<dyn GameState>,
    ) -> Vec<Box<dyn Action>>;
}
