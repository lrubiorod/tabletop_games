use crate::main_code::core::{game_state::GameState, interfaces::printable::IPrintable};
use std::collections::HashSet;

pub trait Action<TGameState: GameState>: IPrintable<TGameState> + Copy {
    /**
     * Executes this action, applying its effect to the given game state. Can access any component IDs stored
     * through the AbstractGameState.getComponentById(int id) method.
     *
     * @param gs - game state which should be modified by this action.
     * @return - true if successfully executed, false otherwise.
     */
    fn execute(&self, gs: TGameState) -> bool;

    /**
     * Returns the string representation of this action from the perspective of the given player.
     * @param gs - game state to be used to generate the string.
     *
     *  May optionally be implemented if Actions are not fully visible
     *  The only impact this has is in the GUI, to avoid this giving too much information to the player.
     *
     * @param perspectivePlayer - player to whom the action should be represented.
     * @return - string representation of this action.
     */
    fn get_string_with_perspective(
        &self,
        game_state: TGameState,
        _perspective_player: usize,
    ) -> String {
        self.get_string(game_state)
    }

    /**
     * The GUI formally supports multiple players viewing a game. This in practice is only going to be used
     * for games with (near) perfect information. For games that actually implement hidden information in
     * a move (Resistance, Hanabi, Sushi Go, etc), we will only need the game actions to implement
     * getString(AbstractGameState, int). This is a helper method to make this downstream imnplementation
     * easier without trying to puzzle out what it means to have multiple players viewing a game with hidden information.
     *
     * @param gs
     * @param perspectiveSet
     * @return
     */
    fn get_string_with_perspective_set(
        &self,
        game_state: TGameState,
        perspective_set: &HashSet<usize>,
    ) -> String {
        let current_player = game_state.current_player();
        let perspective = if perspective_set.contains(&current_player) {
            current_player
        } else {
            *perspective_set.iter().next().unwrap_or(&current_player)
        };
        self.get_string_with_perspective(game_state, perspective)
    }
}
