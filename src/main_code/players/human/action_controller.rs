use crate::main_code::core::actions::action::Action;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

// Define the ActionController
#[derive(Clone)]
pub struct ActionController {
    sender: Sender<Box<dyn Action>>,
    receiver: Arc<Mutex<Receiver<Box<dyn Action>>>>,
    last_action_played: Option<Box<dyn Action>>,
    debug: bool,
}

impl ActionController {
    // Constructor
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        ActionController {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            last_action_played: None,
            debug: false,
        }
    }

    // Set debug mode
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    // Add a new action to the queue from the GUI
    pub fn add_action_from_gui(&self, action: Box<dyn Action>) {
        if let Err(_) = self.sender.send(action) {
            if self.debug {
                println!("Error: Failed to send action to queue.");
            }
        } else if self.debug {
            println!("Action added to ActionController");
        }
    }

    // Get the next action from the queue (blocking)
    pub fn get_action(&mut self) -> Option<Box<dyn Action>> {
        match self.receiver.lock().unwrap().recv() {
            Ok(action) => {
                self.last_action_played = Some(action.clone());
                if self.debug {
                    println!("Action taken via get_action()");
                }
                Some(action)
            }
            Err(_) => {
                if self.debug {
                    println!("Error: No action available.");
                }
                None
            }
        }
    }

    // Return the last action played
    pub fn get_last_action_played(&self) -> Option<&Box<dyn Action>> {
        self.last_action_played.as_ref()
    }

    // Check if there is an available action without blocking
    pub fn has_action(&self) -> bool {
        !self.receiver.lock().unwrap().try_recv().is_err()
    }

    // Reset the controller
    pub fn reset(&mut self) {
        self.last_action_played = None;
        // Note: We don't clear the message channel itself, but reset the last action played.
        if self.debug {
            println!("ActionController reset.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::main_code::core::actions::action::Action;
    use crate::main_code::core::game_state::GameState;

    // Dummy action for testing purposes
    #[derive(Clone)]
    struct TestAction {
        id: i32,
    }

    impl Action for TestAction {
        fn execute(&self, _gs: &mut Box<dyn GameState>) -> bool {
            true
        }

        fn get_string(&self, _gs: &Box<dyn GameState>) -> String {
            format!("TestAction {}", self.id)
        }

        fn id(&self) -> i32 {
            self.id
        }
    }

    #[test]
    fn test_add_action_from_gui() {
        let controller = ActionController::new();
        let action = Box::new(TestAction { id: 1 });

        // Add action to the controller from the "GUI"
        controller.add_action_from_gui(action);

        // Ensure the action can be retrieved
        let retrieved_action = controller.receiver.lock().unwrap().recv().unwrap();
        assert_eq!(retrieved_action.id(), 1);
    }

    #[test]
    fn test_get_action() {
        let mut controller = ActionController::new();
        let action = Box::new(TestAction { id: 2 });

        // Add action to the controller from the "GUI"
        controller.add_action_from_gui(action);

        // Retrieve the action
        let retrieved_action = controller.get_action();
        assert!(retrieved_action.is_some());
        assert_eq!(retrieved_action.unwrap().id(), 2);
    }

    #[test]
    fn test_get_last_action_played() {
        let mut controller = ActionController::new();
        let action = Box::new(TestAction { id: 3 });

        // Add and retrieve an action
        controller.add_action_from_gui(action);
        controller.get_action();

        // Ensure last action played is set correctly
        let last_action = controller.get_last_action_played();
        assert!(last_action.is_some());
        assert_eq!(last_action.unwrap().id(), 3);
    }

    #[test]
    fn test_has_action() {
        let controller = ActionController::new();
        let action = Box::new(TestAction { id: 4 });

        // Initially, there should be no action
        assert!(!controller.has_action());

        // Add an action and check again
        controller.add_action_from_gui(action);
        assert!(controller.has_action());
    }

    #[test]
    fn test_reset() {
        let mut controller = ActionController::new();
        let action = Box::new(TestAction { id: 5 });

        // Add and retrieve an action
        controller.add_action_from_gui(action);
        controller.get_action();

        // Ensure the last action played is set
        assert!(controller.get_last_action_played().is_some());

        // Reset the controller
        controller.reset();

        // Ensure the last action played is now None
        assert!(controller.get_last_action_played().is_none());
    }
}
