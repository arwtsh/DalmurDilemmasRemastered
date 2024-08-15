use crate::event_system::event_manager::get_event_system;
use crate::event_system::events::EventType::MoveLeft;

use crate::command_system::commands::Command;
use crate::scene_system::scene_manager::get_mut_scene_manager;

/// A way for the player to move locations.
/// Moves the player to the scene marked "left"
pub struct CommandLook;

impl Command for CommandLook {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "look".to_string(),
            "LOOK".to_string(),
            "Look".to_string(),
            "l".to_string(),
            "L".to_string()
        ]
    }

    fn call_command(&self, _params: &String) {
        get_mut_scene_manager().look_current_scene();
    }
}