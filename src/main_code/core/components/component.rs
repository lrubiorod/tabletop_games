use crate::main_code::core::core_constants::ComponentType;
use downcast_rs;
use dyn_clone;
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::atomic::{AtomicUsize, Ordering},
};

// Atomic counter for unique component IDs
static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

pub trait Component: dyn_clone::DynClone + downcast_rs::Downcast {
    fn component_id(&self) -> usize;
    fn nested_components(&self) -> Vec<Box<dyn Component>>;
}
dyn_clone::clone_trait_object!(Component);
downcast_rs::impl_downcast!(Component);

impl fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[impl Component: {}]", self.component_id())
    }
}

#[derive(Default, Debug, Clone)]
pub struct BaseComponent {
    // Unique ID for this component
    component_id: usize,
    // Type of this component
    component_type: ComponentType,
    // By default, belongs to the game (owner is -1)
    owner_id: i32,
    // Name of this component
    component_name: String,
}

impl BaseComponent {
    // Constructor that takes only the type (default name will be the type's string representation)
    pub fn new(component_type: ComponentType) -> Self {
        Self {
            component_id: GLOBAL_ID.fetch_add(1, Ordering::Relaxed),
            component_name: component_type.to_string(),
            component_type,
            owner_id: -1,
        }
    }

    // Constructor that takes the type and name of the component
    pub fn new_with_name(component_type: ComponentType, name: &str) -> Self {
        Self {
            component_id: GLOBAL_ID.fetch_add(1, Ordering::Relaxed),
            component_type,
            owner_id: -1,
            component_name: name.to_string(),
        }
    }

    // Constructor that takes type, name, and a specific component ID
    pub fn new_with_name_and_id(
        component_type: ComponentType,
        name: &str,
        component_id: usize,
    ) -> Self {
        Self {
            component_id,
            component_type,
            owner_id: -1,
            component_name: name.to_string(),
        }
    }

    // Constructor that takes type and a specific component ID (default name will be the type's string representation)
    pub fn new_with_id(component_type: ComponentType, component_id: usize) -> Self {
        Self {
            component_id,
            component_name: component_type.to_string(),
            component_type,
            owner_id: -1,
        }
    }

    pub fn component_type(&self) -> ComponentType {
        self.component_type.clone()
    }

    pub fn owner_id(&self) -> i32 {
        self.owner_id
    }

    pub fn set_owner_id(&mut self, owner_id: i32) {
        self.owner_id = owner_id;
    }

    pub fn component_name(&self) -> &str {
        &self.component_name
    }

    pub fn set_component_name(&mut self, name: String) {
        self.component_name = name;
    }
}

impl Component for BaseComponent {
    fn component_id(&self) -> usize {
        self.component_id
    }

    fn nested_components(&self) -> Vec<Box<dyn Component>> {
        vec![]
    }
}

impl Hash for BaseComponent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.component_id.hash(state);
    }
}

impl fmt::Display for BaseComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.component_type)
    }
}

impl PartialEq for BaseComponent {
    fn eq(&self, other: &Self) -> bool {
        self.component_id == other.component_id
    }
}

impl Eq for BaseComponent {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_different_ids() {
        let t1 = BaseComponent::new(ComponentType::Token);
        assert_eq!(t1.component_id(), 0);
        let t2 = BaseComponent::new(ComponentType::Token);
        assert_eq!(t2.component_id(), 1);
        assert_ne!(t1, t2);
    }
}
