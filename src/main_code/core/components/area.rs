use crate::main_code::core::{
    components::component::{BaseComponent, Component},
    core_constants::{ComponentType, VisibilityMode},
    interfaces::component_container::ComponentContainer,
};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
pub struct Area {
    base: BaseComponent,
    components: HashMap<usize, Box<dyn Component>>,
}

impl Area {
    pub fn new(owner: i32) -> Self {
        let mut base = BaseComponent::new_with_name(ComponentType::Area, "");
        base.set_owner_id(owner);
        Self {
            base,
            components: HashMap::default(),
        }
    }

    pub fn new_with_id(owner: i32, id: usize) -> Self {
        let mut base = BaseComponent::new_with_name_and_id(ComponentType::Area, "", id);
        base.set_owner_id(owner);
        Self {
            base,
            components: HashMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.components.clear()
    }

    pub fn components_map(&self) -> HashMap<usize, Box<dyn Component>> {
        self.components.clone()
    }

    pub fn nested_keys(&self) -> Vec<usize> {
        self.components.keys().cloned().collect()
    }

    pub fn get_component(&self, key: usize) -> Option<&Box<dyn Component>> {
        self.components.get(&key)
    }

    pub fn put_component(&mut self, c: Box<dyn Component>) -> Option<Box<dyn Component>> {
        for nc in c.nested_components() {
            self.components.insert(nc.component_id(), nc.clone());
        }
        self.components.insert(c.component_id(), c)
    }

    pub fn put_components(&mut self, components: Vec<Box<dyn Component>>) {
        for c in components {
            self.put_component(c);
        }
    }
}

impl Component for Area {
    fn component_id(&self) -> usize {
        self.base.component_id()
    }

    fn nested_components(&self) -> Vec<Box<dyn Component>> {
        self.get_components()
    }
}

impl ComponentContainer for Area {
    fn get_components(&self) -> Vec<Box<dyn Component>> {
        self.components.values().cloned().collect()
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

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        if self.component_id() != other.component_id() {
            return false;
        }

        let mut sorted_keys_self = self.nested_keys();
        let mut sorted_keys_other = other.nested_keys();
        sorted_keys_self.sort_unstable();
        sorted_keys_other.sort_unstable();

        sorted_keys_self == sorted_keys_other
    }
}

impl Eq for Area {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::main_code::core::components::token::Token;

    #[test]
    fn test_put_components_and_clear() {
        let c1 = Token::new_with_id("Meeple1", 1);
        let c2 = Token::new_with_id("Meeple2", 2);
        let c3 = Token::new_with_id("Meeple3", 3);

        let mut area = Area::new_with_id(-1, 10);
        area.put_components(vec![
            Box::new(c1.clone()),
            Box::new(c2.clone()),
            Box::new(c3.clone()),
        ]);
        assert_eq!(area.get_size(), 3);

        let area_c1 = area.get_component(1).unwrap();
        let area_c2 = area.get_component(2).unwrap();
        let area_c3 = area.get_component(3).unwrap();

        assert_eq!(c1, *area_c1.downcast_ref::<Token>().unwrap());
        assert_eq!(c2, *area_c2.downcast_ref::<Token>().unwrap());
        assert_eq!(c3, *area_c3.downcast_ref::<Token>().unwrap());

        area.clear();

        assert!(area.get_component(1).is_none());
        assert!(area.get_component(2).is_none());
        assert!(area.get_component(3).is_none());
    }

    #[test]
    fn test_put_components_with_nested_components() {
        let c1 = Token::new_with_id("Meeple1", 1);
        let c2 = Token::new_with_id("Meeple2", 2);
        let c3 = Token::new_with_id("Meeple3", 3);
        let mut a1 = Area::new_with_id(-1, 10);
        a1.put_components(vec![
            Box::new(c1.clone()),
            Box::new(c2.clone()),
            Box::new(c3.clone()),
        ]);

        let c4 = Token::new_with_id("Meeple4", 4);
        let c5 = Token::new_with_id("Meeple5", 5);
        let c6 = Token::new_with_id("Meeple6", 6);
        let mut a2 = Area::new_with_id(-1, 20);
        a2.put_components(vec![
            Box::new(c4.clone()),
            Box::new(c5.clone()),
            Box::new(c6.clone()),
        ]);

        let c7 = Token::new_with_id("Meeple7", 7);
        let mut a3 = Area::new_with_id(-1, 30);
        a3.put_components(vec![
            Box::new(a1.clone()),
            Box::new(a2.clone()),
            Box::new(c7.clone()),
        ]);
        assert_eq!(a3.get_size(), 9);

        let a3c1 = a3.get_component(1).unwrap();
        let a3c2 = a3.get_component(2).unwrap();
        let a3c3 = a3.get_component(3).unwrap();
        let a3c4 = a3.get_component(4).unwrap();
        let a3c5 = a3.get_component(5).unwrap();
        let a3c6 = a3.get_component(6).unwrap();

        assert_eq!(c1, *a3c1.downcast_ref::<Token>().unwrap());
        assert_eq!(c2, *a3c2.downcast_ref::<Token>().unwrap());
        assert_eq!(c3, *a3c3.downcast_ref::<Token>().unwrap());
        assert_eq!(c4, *a3c4.downcast_ref::<Token>().unwrap());
        assert_eq!(c5, *a3c5.downcast_ref::<Token>().unwrap());
        assert_eq!(c6, *a3c6.downcast_ref::<Token>().unwrap());

        let a3a1 = a3
            .get_component(10)
            .unwrap()
            .downcast_ref::<Area>()
            .unwrap()
            .clone();
        assert_eq!(a3a1.get_size(), 3);
        assert!(a3a1.get_component(1).is_some());
        assert!(a3a1.get_component(2).is_some());
        assert!(a3a1.get_component(3).is_some());
        assert!(a3a1.get_component(4).is_none());
        assert!(a3a1.get_component(5).is_none());
        assert!(a3a1.get_component(6).is_none());

        let a3a2 = a3
            .get_component(20)
            .unwrap()
            .downcast_ref::<Area>()
            .unwrap()
            .clone();
        assert_eq!(a3a2.get_size(), 3);
        assert!(a3a2.get_component(1).is_none());
        assert!(a3a2.get_component(2).is_none());
        assert!(a3a2.get_component(3).is_none());
        assert!(a3a2.get_component(4).is_some());
        assert!(a3a2.get_component(5).is_some());
        assert!(a3a2.get_component(6).is_some());
    }
}
