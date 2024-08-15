use std::collections::{HashMap, HashSet};
use log::error;

use crate::command_system::commands::CommandId;
use std::slice::Iter;

use super::commands::Command;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CommandSchemes {
    MainMenu,
    Gameplay,
    ProfileSelect
}

impl CommandSchemes {
    pub fn is_command_member(&self, command: &CommandId) -> bool {
        *command != CommandId::None && self.get_scheme_commands().get(&command).is_some()
    }

    /// Connect a set of strings to commands.
    /// This is useful for interpreting player input.
    pub fn get_scheme_parses(&self, all_commands: &HashMap<CommandId, Box<dyn Command>>) -> HashMap<String, CommandId> {
        let mut string_parses: HashMap<String, CommandId> = HashMap::new();

        //Go through each command in the scheme.
        for command_id in self.get_scheme_commands() {
            //Get the contents of the command from the ID
            if let Some(command) = all_commands.get(&command_id) {
                //Go through each identifier for the command
                for command_identifier in command.as_ref().get_identifiers() {
                    if string_parses.contains_key(&command_identifier) {
                        error!("Scheme {} didn't properly compile it's command parses. A command parse identifier \"{}\" in command {} already points to command {}", self.to_string(), command_identifier, command_id.to_string(), string_parses.get(&command_identifier).unwrap().to_string());
                    }
                    else {
                        //Populate the scene_parse hash map from the data in scene data.
                        string_parses.insert(command_identifier, command_id);
                    }   
                }
            } else {
                error!("Scheme {} didn't properly compile it's command parses. The command ID \"{}\" does not have any data associated with it.", self.to_string(), command_id.to_string());
            }

            
        }

        string_parses
    }

    fn get_scheme_commands(&self) -> HashSet<CommandId> {
        match self {
            CommandSchemes::MainMenu => {
                HashSet::from_iter(vec![
                    CommandId::Exit,
                    CommandId::Play,
                    CommandId::Credits,
                    CommandId::Profile,
                    CommandId::Help
                ])
            },
            CommandSchemes::Gameplay => {
                HashSet::from_iter(vec![
                    CommandId::Help,
                    CommandId::Return,
                    CommandId::Look,
                    CommandId::Examine,
                    CommandId::Grab,
                    CommandId::Inventory,
                    CommandId::Inspect,
                    CommandId::Use,
                    CommandId::Puzzle,
                    CommandId::Move
                ])
            },
            CommandSchemes::ProfileSelect => {
                HashSet::from_iter(vec![
                    CommandId::ProfileDelete,
                    CommandId::ProfileLoad,
                    CommandId::ProfileNew,
                    CommandId::ProfileCancel,
                    CommandId::Help
                ])
            }
        }
    }

    pub fn get_scheme_help_text(&self) -> &str {
        match self {
            CommandSchemes::MainMenu => concat!(
                "HELP will tell you the commands you can use.",
                "\nPLAY will start the game.",
                "\nPROFILE allows you switch between saves of the game.",
                "\nCREDITS displays the creator of the game.",
                "\nQUIT will exit the game to desktop."),
            CommandSchemes::ProfileSelect => concat!(
                "LOAD + selection loads the new profile.",
                "\nDELETE + selection will delete that profile.",
                "\nNEW creates a new profile.",
                "\nCANCEL returns to the main menu."),
            CommandSchemes::Gameplay => concat!(
                "You will be given a text description of a scene.",
		        "\nImportant or interactable objects in the scene will be CAPITALIZED.",
		        "\nThere are a list of commands you can type to interact with the scene.",
		        "\nCommands are not case-sensitive. Here they are capitalized to stand out against their description.",
		        "\nMOVE + ADJACENT LOCATION changes which scene is being described. You can only move to locations mentioned in the scene.",
		        "\nGRAB + LISTED ITEM adds an item listed in the scene into your inventory.",
		        "\nINVENTORY lists all the items you have in your inventory.",
		        "\nUSE + ITEM IN INVENTORY + ON + OBJECT IN SCENE will (if correct) progress the scene and possibly remove the item from your inventory.",
		        "\nEXAMINE + OBJECT IN SCENE will describe the specified object in more detail.",
		        "\nINSPECT + ITEM IN INVENTORY will describe the item, potentially revealing hidden clues.",
		        "\nLOOK will repeat the description of the location.",
		        "\nHELP repeats these tips.",
		        "\nEXIT closes the game.",
		        "\nOccasionally a puzzle will have unique commands. The puzzle will list those commands explicitly.")
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            CommandSchemes::MainMenu => "Main Menu",
            CommandSchemes::Gameplay => "Gameplay",
            CommandSchemes::ProfileSelect => "Profile Select"
        }
    }

    /// Get an iterator of all the commands.
    /// This is useful for initializing all the commands at game start.
    /// This does not include the None command.
    pub fn iter() -> Iter<'static, CommandSchemes> {
        static COMMANDS: [CommandSchemes; 3] = [
            CommandSchemes::Gameplay,
            CommandSchemes::MainMenu,
            CommandSchemes::ProfileSelect
        ];
        COMMANDS.iter()
    }
}

