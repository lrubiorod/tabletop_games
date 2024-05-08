use crate::main_code::core::{
    components::component::Component, core_constants::VisibilityMode,
    interfaces::component_container::ComponentContainer,
};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Area {
    components: HashMap<usize, Component>,
}

impl Area {
    pub fn clear(&mut self) {
        self.components.clear()
    }

    pub fn components_map(&self) -> HashMap<usize, Component> {
        self.components.clone()
    }

    pub fn get_component(&self, key: usize) -> Option<&Component> {
        self.components.get(&key)
    }

    pub fn put_component(&mut self, c: Component) -> Option<Component> {
        if let Some(nested_components) = c.get_components() {
            for nc in nested_components {
                self.components.insert(nc.component_id(), nc.clone());
            }
        }
        self.components.insert(c.component_id(), c)
    }

    pub fn put_components(&mut self, components: Vec<Component>) {
        for c in components {
            self.put_component(c);
        }
    }
}

impl ComponentContainer<Component> for Area {
    fn get_components(&self) -> Vec<&Component> {
        self.components.values().collect()
    }

    fn get_visibility_mode(&self) -> VisibilityMode {
        VisibilityMode::VisibleToAll
    }
}

impl Hash for Area {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for key in self.components.keys() {
            key.hash(state);
        }
    }
}
