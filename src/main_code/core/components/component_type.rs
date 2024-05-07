use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ComponentType {
    Area,
    Token,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            ComponentType::Area => "Area",
            ComponentType::Token => "Token",
        };
        write!(f, "{}", name)
    }
}
