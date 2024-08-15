use crate::event_system::event_manager::get_event_system;
use crate::event_system::events::EventType::MoveRight;
use crate::command_system::commands::Command;
use crate::inventory_system::items;
use crate::save_system::save_system::get_mut_save_system;
use crate::scene_system::scene_manager::get_mut_scene_manager;

pub struct CommandInspect;

impl Command for CommandInspect {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "inspect".to_string(),
            "INSPECT".to_string(),
            "Inspect".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        if let Some(item) = items::parse_item(_params) {
            if get_mut_save_system().is_item_in_inventory(item) {
                println!("{}", item.get_description());
            } else {
                println!("You do not have that item.");
            }
        } else {
            println!("No item of that name was found. Please check for typos.");
        }
    }
}