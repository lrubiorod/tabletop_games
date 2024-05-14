pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }
    /*
       pub fn run_one(game_to_play: GameType, players: Vec<u8>, seed: u64) -> Game {
           let game = Self::new();

           game
       }

           public static Game runOne(GameType gameToPlay, String parameterConfigFile, List<AbstractPlayer> players, long seed,
           boolean randomizeParameters, List<IGameListener> listeners, ActionController ac, int turnPause) {
    */
    pub fn main(&self) {
        println!("Ejecutando el juego...");
    }
}
