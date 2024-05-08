use crate::main_code::core::{components::component::Component, core_constants::VisibilityMode};

/// A trait to be used on any Component that contains other Components.
/// The trait is 'read-only', and deliberately avoids specifying add/remove type methods.
/// The purposes are:
///
/// i) To be used to gather information about game states for game metrics and comparisons.
/// ii) To indicate who can see the contents of the Container (Everyone, No-one, just the Owner).
/// iii) As a holder of useful stream-related default methods - these are all read-only methods.
pub trait ComponentContainer<Component> {
    /// Returns a vector of all the Components in the Container
    fn get_components(&self) -> &Vec<Component>;

    /// Returns the visibility mode of the Container
    fn get_visibility_mode(&self) -> VisibilityMode;

    /// Returns the size of the container (number of components)
    fn get_size(&self) -> usize {
        self.get_components().len()
    }
}
