use crate::event_system::event_manager::get_event_system;
use crate::event_system::events::EventType::MoveRight;
use crate::command_system::commands::Command;
use crate::scene_system::scene_manager::get_mut_scene_manager;

/// A way for the player to move locations.
/// Moves the player to the scene marked "right"
pub struct CommandExamine;

impl Command for CommandExamine {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "examine".to_string(),
            "EXAMINE".to_string(),
            "Examine".to_string(),
            "e".to_string(),
            "E".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        get_mut_scene_manager().examine_in_current_scene(_params);
    }
}