#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ComponentType {
    Area,
    Token,
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ComponentType::Area => "Area",
            ComponentType::Token => "Token",
        };
        write!(f, "{}", name)
    }
}

impl Default for ComponentType {
    fn default() -> Self {
        ComponentType::Token
    }
}

/**
 * Used in Components that contain other Components (see ComponentContainer) to mark which players can see the
 * contents.
 * MixedVisibility is an indicator that none of the previous three apply, and that the ComponentContainer
 * will need to implement more sophisticated logic. This is done for example in PartialObservableDeck - and
 * this should cover almost all future eventualities.
 */
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum VisibilityMode {
    VisibleToAll,
    HiddenToAll,
    VisibleToOwner,
    FirstVisibleToAll,
    LastVisibleToAll,
    MixedVisibility,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum GameResult {
    WinGame,
    WinRound,
    DrawGame,
    DrawRound,
    LoseRound,
    LoseGame,
    Disqualify,
    Timeout,
    GameOngoing,
    GameEnd,
}

impl GameResult {
    pub fn value(&self) -> i8 {
        match self {
            GameResult::WinGame => 1,
            GameResult::WinRound => 0,
            GameResult::DrawGame => 0,
            GameResult::DrawRound => 0,
            GameResult::LoseRound => 0,
            GameResult::LoseGame => -1,
            GameResult::Disqualify => -2,
            GameResult::Timeout => -3,
            GameResult::GameOngoing => 0,
            GameResult::GameEnd => 3,
        }
    }
}

impl Default for GameResult {
    fn default() -> Self {
        GameResult::GameOngoing
    }
}
