use crate::event_system::event_manager::get_event_system;
use crate::event_system::events::EventType::MoveRight;
use crate::command_system::commands::Command;
use crate::save_system::save_system::get_mut_save_system;
use crate::scene_system::scene_manager::get_mut_scene_manager;

pub struct CommandInventory;

impl Command for CommandInventory {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "inventory".to_string(),
            "INVENTORY".to_string(),
            "Inventory".to_string(),
            "i".to_string(),
            "I".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        println!("You have these items in your inventory: ");

        for item in get_mut_save_system().get_inventory_iter() {
            println!("{}", item.to_string());
        }
    }
}