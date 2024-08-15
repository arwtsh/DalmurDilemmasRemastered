use crate::{command_system::commands::Command, event_system::{event_manager::get_event_system, events::EventType}, inventory_system::items, save_system::save_system::get_mut_save_system};
use crate::scene_system::scene_manager::get_mut_scene_manager;

/// Returns to the main menu
pub struct CommandUse;

impl Command for CommandUse {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "use".to_string(),
            "USE".to_string(),
            "Use".to_string(),
            "u".to_string(),
            "U".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        if let Some(split) = _params.trim().split_once(" on ") {
            if let Some(item) = items::parse_item(&split.0.to_string()) {
                if get_mut_save_system().is_item_in_inventory(item) {
                    get_mut_scene_manager().use_item_in_current_scene(item, &split.1.to_string());
                } else {
                    println!("You do not have that item.");
                }
            } else {
                println!("That item does not exist.");
            }
        } else {
            println!("Use command not formatted correctly.");
        }
    }
}