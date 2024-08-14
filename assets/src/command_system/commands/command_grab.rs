use crate::{command_system::commands::Command, event_system::{event_manager::get_event_system, events::EventType}, inventory_system::items};
use crate::scene_system::scene_manager::get_mut_scene_manager;

/// Returns to the main menu
pub struct CommandGrab;

impl Command for CommandGrab {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "grab".to_string(),
            "GRAB".to_string(),
            "Grab".to_string(),
            "g".to_string(),
            "G".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        if let Some(item) = items::parse_item(_params) {
            get_mut_scene_manager().grab_item_in_current_scene(item);
        } else {
            println!("That item does not exist.");
        }
    }
}