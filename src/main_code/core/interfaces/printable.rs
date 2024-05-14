use crate::main_code::core::game_state::GameState;
use std::fmt;

pub trait IPrintable<TGameState: GameState>: fmt::Display + fmt::Debug {
    /**
     * Retrieves a short string for this object, given an GameState for context.
     * @param gameState - game state provided for context.
     * @return - short String
     */
    fn get_string(&self, _game_state: TGameState) -> String {
        self.to_string()
    }

    /**
     * Prints itself to console.
     */
    fn print_to_console(&self) {
        println!("{}", self.to_string());
    }
}
