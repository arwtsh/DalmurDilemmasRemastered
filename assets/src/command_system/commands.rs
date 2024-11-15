pub mod command_credits;
pub mod command_exit;
pub mod command_help;
pub mod command_look;
pub mod command_play;
pub mod command_profile;
pub mod command_examine;
pub mod command_profile_delete;
pub mod command_profile_load;
pub mod command_profile_new;
pub mod command_profile_cancel;
pub mod command_return;
pub mod command_grab;
pub mod command_inventory;
pub mod command_inspect;
pub mod command_use;
pub mod command_puzzle;
pub mod command_move;

use std::slice::Iter;

///An ID for commands
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CommandId {
    None,
    Exit,
    Help,
    Play,
    Credits,
    Profile,
    ProfileDelete,
    ProfileNew,
    ProfileLoad,
    ProfileCancel,
    Return,
    Look,
    Examine,
    Grab,
    Inventory,
    Inspect,
    Use,
    Puzzle,
    Move
}

impl CommandId {
    /// Convert the command ID to a string.
    /// This is useful for debugging.
    pub fn to_string(&self) -> &str {
        match *self {
            CommandId::None => "None",
            CommandId::Exit => "Exit",
            CommandId::Help => "Help",
            CommandId::Credits => "Credits",
            CommandId::Play => "Play",
            CommandId::Profile => "Profile",
            CommandId::ProfileDelete => "ProfileDelete",
            CommandId::ProfileNew => "ProfileNew",
            CommandId::ProfileLoad => "ProfileLoad",
            CommandId::ProfileCancel => "ProfileCancel",
            CommandId::Return => "ReturnToMainMenu",
            CommandId::Look => "Look",
            CommandId::Examine => "Examine",
            CommandId::Grab => "Grab",
            CommandId::Inventory => "Inventory",
            CommandId::Inspect => "Inspect",
            CommandId::Use => "Use",
            CommandId::Puzzle => "Puzzle",
            CommandId::Move => "Move"
        }
    }

    /// Get the command associated with the ID.
    pub fn get_command(&self) -> Box<dyn Command> {
        match *self {
            CommandId::None => panic!("Command \"None\" is not a command, it is a placeholder that represents no command."),
            CommandId::Exit => Box::new(command_exit::CommandExit),
            CommandId::Help => Box::new(command_help::CommandHelp),
            CommandId::Credits => Box::new(command_credits::CommandCredits),
            CommandId::Play => Box::new(command_play::CommandPlay),
            CommandId::Profile => Box::new(command_profile::CommandProfile),
            CommandId::ProfileDelete => Box::new(command_profile_delete::CommandProfileDelete),
            CommandId::ProfileLoad => Box::new(command_profile_load::CommandProfileLoad),
            CommandId::ProfileNew => Box::new(command_profile_new::CommandProfileNew),
            CommandId::ProfileCancel => Box::new(command_profile_cancel::CommandProfileCancel),
            CommandId::Return => Box::new(command_return::CommandReturn),
            CommandId::Look => Box::new(command_look::CommandLook),
            CommandId::Examine => Box::new(command_examine::CommandExamine),
            CommandId::Grab => Box::new(command_grab::CommandGrab),
            CommandId::Inventory => Box::new(command_inventory::CommandInventory),
            CommandId::Inspect => Box::new(command_inspect::CommandInspect),
            CommandId::Use => Box::new(command_use::CommandUse),
            CommandId::Puzzle => Box::new(command_puzzle::CommandPuzzle),
            CommandId::Move => Box::new(command_move::CommandMove)
        }
    }

    /// Get an iterator of all the commands.
    /// This is useful for initializing all the commands at game start.
    /// This does not include the None command.
    pub fn iter() -> Iter<'static, CommandId> {
        static COMMANDS: [CommandId; 18] = [
            CommandId::Exit,
            CommandId::Help,
            CommandId::Credits,
            CommandId::Play,
            CommandId::Profile,
            CommandId::ProfileDelete,
            CommandId::ProfileLoad,
            CommandId::ProfileNew,
            CommandId::ProfileCancel,
            CommandId::Return,
            CommandId::Look,
            CommandId::Examine,
            CommandId::Grab,
            CommandId::Inventory,
            CommandId::Inspect,
            CommandId::Use,
            CommandId::Puzzle,
            CommandId::Move
        ];
        COMMANDS.iter()
    }
}

/// Declares this as a command the player can call.
pub trait Command { //A trait makes a struct act more like a class with OOP.
    ///Get the string identities of this command.
    ///These will be used to match player text imput to a specific command.
    fn get_identifiers(&self) -> Vec<String>;

    ///Call the logic of this command.
    fn call_command(&self, params: &String);
}