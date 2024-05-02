pub enum GameType {
    // Template
    Template,
    // Rest of games
    EasyBoop,
    //......
}

pub struct GameAttributes<TGameState, TForwardModel, TParameters, TGUIManager> {
    min_players: u32,
    max_players: u32,
    categories: Vec<GameCategory>,
    mechanics: Vec<GameMechanic>,
    data_path: Option<String>,
    game_state: TGameState,
    forward_model: TForwardModel,
    parameters: TParameters,
    gui_manager: TGUIManager,
}

impl<TGameState, TForwardModel, TParameters, TGUIManager>
    GameAttributes<TGameState, TForwardModel, TParameters, TGUIManager>
{
    pub fn new(
        min_players: u32,
        max_players: u32,
        categories: Vec<GameCategory>,
        mechanics: Vec<GameMechanic>,
        data_path: Option<String>,
        game_state: TGameState,
        forward_model: TForwardModel,
        parameters: TParameters,
        gui_manager: TGUIManager,
    ) -> Self {
        GameAttributes {
            min_players,
            max_players,
            categories,
            mechanics,
            data_path,
            game_state,
            forward_model,
            parameters,
            gui_manager,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameCategory {
    Abstract,
    Animals,
    Simple,
    Strategy,
}

#[derive(Debug, Clone)]
pub enum GameMechanic {
    DiceRolling,
    HandManagement,
    PatternBuilding,
    PushYourLuck,
}
