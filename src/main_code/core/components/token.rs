use crate::main_code::core::{
    components::component::{BaseComponent, Component},
    core_constants::ComponentType,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Token {
    base: BaseComponent,
}

impl Component for Token {}

impl Token {
    pub fn new(name: String) -> Self {
        Self {
            base: BaseComponent::new_with_name(ComponentType::Token, name),
        }
    }

    pub fn new_with_id(name: String, component_id: usize) -> Self {
        Self {
            base: BaseComponent::new_with_name_and_id(ComponentType::Token, name, component_id),
        }
    }
    pub fn token_type(&self) -> &str {
        &self.base.component_name()
    }
}
