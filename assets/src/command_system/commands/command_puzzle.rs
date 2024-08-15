use crate::{command_system::commands::Command, event_system::{event_manager::get_event_system, events::EventType}, inventory_system::items, save_system::save_system::get_mut_save_system};
use crate::scene_system::scene_manager::get_mut_scene_manager;

/// Returns to the main menu
pub struct CommandPuzzle;

impl Command for CommandPuzzle {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "puzzle".to_string(),
            "PUZZLE".to_string(),
            "Puzzle".to_string(),
            "p".to_string(),
            "P".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        if let Some(split) = _params.trim().split_once(" is ") {
            get_mut_scene_manager().solve_puzzle_in_current_scene(&split.0.to_string(), &split.1.to_string());
        } else {
            println!("Puzzle command not formatted correctly.");
        }
    }
}