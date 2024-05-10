use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Token {
    token_type: String,
}

impl Token {
    pub fn new(token_type: &str) -> Self {
        Self { token_type: token_type.to_string() }
    }
    pub fn token_type(&self) -> &str {
        &self.token_type
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}
