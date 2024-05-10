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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::main_code::core::components::token::Token;
    use crate::main_code::core::core_constants::ComponentType;

    fn create_token_component(name: &str, id: usize) -> Component {
        Component::new_with_id(ComponentType::Token(Token::new(name)), id)
    }

    fn create_area_component(id: usize, area: Area) -> Component {
        Component::new_with_id(ComponentType::Area(area), id)
    }

    #[test]
    fn test_put_components_and_clear() {
        let component_1 = create_token_component("Meeple1", 1);
        let component_2 = create_token_component("Meeple2", 2);
        let component_3 = create_token_component("Meeple3", 3);

        let mut area = Area::default();
        area.put_components(vec![component_1.clone(),component_2.clone(), component_3.clone()]);
        assert_eq!(area.get_size(), 3);

        let area_component_1 = area.get_component(1).unwrap();
        let area_component_2 = area.get_component(2).unwrap();
        let area_component_3 = area.get_component(3).unwrap();

        assert_eq!(component_1, *area_component_1);
        assert_eq!(component_2, *area_component_2);
        assert_eq!(component_3, *area_component_3);

        area.clear();

        assert!(area.get_component(1).is_none());
        assert!(area.get_component(2).is_none());
        assert!(area.get_component(3).is_none());
    }


    #[test]
    fn test_put_components_with_nested_components() {
        let component_1 = create_token_component("Meeple1", 1);
        let component_2 = create_token_component("Meeple2", 2);
        let component_3 = create_token_component("Meeple3", 3);
        let mut area_1 = Area::default();
        area_1.put_components(vec![component_1.clone(),component_2.clone(), component_3.clone()]);
        let comp_area_1 = create_area_component(10, area_1);

        let component_4 = create_token_component("Meeple4", 4);
        let component_5 = create_token_component("Meeple5", 5);
        let component_6 = create_token_component("Meeple6", 6);
        let mut area_2 = Area::default();
        area_2.put_components(vec![component_4.clone(),component_5.clone(), component_6.clone()]);
        let comp_area_2 = create_area_component(20, area_2);

        let mut big_area = Area::default();
        big_area.put_components(vec![comp_area_1.clone(), comp_area_2.clone()]);
        assert_eq!(big_area.get_size(), 8);

        let area_component_1 = big_area.get_component(1).unwrap();
        let area_component_2 = big_area.get_component(2).unwrap();
        let area_component_3 = big_area.get_component(3).unwrap();
        let area_component_4 = big_area.get_component(4).unwrap();
        let area_component_5 = big_area.get_component(5).unwrap();
        let area_component_6 = big_area.get_component(6).unwrap();

        assert_eq!(component_1, *area_component_1);
        assert_eq!(component_2, *area_component_2);
        assert_eq!(component_3, *area_component_3);
        assert_eq!(component_4, *area_component_4);
        assert_eq!(component_5, *area_component_5);
        assert_eq!(component_6, *area_component_6);

        if let ComponentType::Area(area) = big_area.get_component(10).unwrap().component_type(){
            assert_eq!(area.get_size(), 3);
            assert!(area.get_component(1).is_some());
            assert!(area.get_component(2).is_some());
            assert!(area.get_component(3).is_some());
            assert!(area.get_component(4).is_none());
            assert!(area.get_component(5).is_none());
            assert!(area.get_component(6).is_none());
        } else {
            unreachable!("Area 1 does not exist")
        }

        if let ComponentType::Area(area) = big_area.get_component(20).unwrap().component_type(){
            assert_eq!(area.get_size(), 3);
            assert!(area.get_component(1).is_none());
            assert!(area.get_component(2).is_none());
            assert!(area.get_component(3).is_none());
            assert!(area.get_component(4).is_some());
            assert!(area.get_component(5).is_some());
            assert!(area.get_component(6).is_some());
        } else {
            unreachable!("Area 2 does not exist")
        }
    }
}

