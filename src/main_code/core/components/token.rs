use crate::main_code::core::{
    components::component::{BaseComponent, Component},
    core_constants::ComponentType,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Token {
    base: BaseComponent,
    token_type: String,
}

impl Token {
    pub fn new(token_type: &str) -> Self {
        Self {
            token_type: token_type.to_string(),
            base: BaseComponent::new_with_name(ComponentType::Token, token_type),
        }
    }
    pub fn new_with_id(token_type: &str, id: usize) -> Self {
        Self {
            token_type: token_type.to_string(),
            base: BaseComponent::new_with_name_and_id(ComponentType::Token, token_type, id),
        }
    }
    pub fn token_type(&self) -> &str {
        &self.token_type
    }
}

impl Component for Token {
    fn component_id(&self) -> usize {
        self.base.component_id()
    }

    fn nested_components(&self) -> Vec<Box<dyn Component>> {
        vec![]
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_type)
    }
}
