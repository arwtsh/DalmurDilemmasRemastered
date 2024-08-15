use crate::event_system::event_manager::get_event_system;
use crate::event_system::events::EventType::MoveRight;
use crate::command_system::commands::Command;
use crate::save_system::save_system::get_mut_save_system;
use crate::scene_system::scene_id::SceneId;
use crate::scene_system::scene_manager::{get_mut_scene_manager, get_scene_manager};

pub struct CommandMove;

impl Command for CommandMove {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "move".to_string(),
            "MOVE".to_string(),
            "Move".to_string(),
            "m".to_string(),
            "M".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        let scene = get_scene_manager().parse_scene(_params.clone());
        if scene != SceneId::None {
            get_mut_scene_manager().move_command(scene);
        } else {
            println!("That room may not be adjacent or you may have misspelled something.");
        }
    }
}