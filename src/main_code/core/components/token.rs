#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Token {
    token_type: String,
}

impl Token {
    pub fn new(token_type: String) -> Self {
        Self { token_type }
    }
    pub fn token_type(&self) -> &str {
        &self.token_type()
    }
}
