use crate::main_code::core::components::{area::Area, token::Token};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ComponentType {
    Area(Area),
    Token(Token),
}

impl Display for ComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ComponentType::Area(_) => "Area",
            ComponentType::Token(_) => "Token",
        };
        write!(f, "{}", name)
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