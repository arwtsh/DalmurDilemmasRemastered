use crate::{event_system::event_manager::EventSystem, inventory_system::items::ItemId, save_system::save_system::SaveSystem, scene_system::scene_id::SceneId};

/// Data that is stored by the scene manager at game initialization.
/// It can be accessed even when the item isn't loaded.
pub struct SceneData {
    pub identifiers: Vec<String>, //All strings that could be referencing this Scene.
    pub id: SceneId, //The Scene enum that is an ID for this scene.
}

/// Declares this as a scene that can be loaded
pub trait Scene {
    /// Invoked when this scene is unloaded from memory.
    fn unload(&self, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) {}
    /// Invoked when the player exits this scene.
    fn exit_scene(&self, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) {}
    /// Invoked when the player enters this scene.
    fn enter_scene(&self, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) {}
    /// Invoked when the player looks at the room
    fn display_room_info(&self, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) {}
    /// Invoked when the player tries to examine something
    fn examine(&self, _examinable: &String,_event_system: &mut EventSystem, _save_system: &mut SaveSystem) {}
    /// Invoked when the player tries to examine something
    fn grab_item(&self, item: &ItemId,_event_system: &mut EventSystem, _save_system: &mut SaveSystem) {}
    /// Use an item on something interactable in the scene
    fn use_item(&self, _item: &ItemId, _target: &String, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) {}
    /// Unique puzzle system
    fn puzzle(&self, _puzzle: &String, _solution: &String, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) {}
    /// Check if the scene can move to that position.
    fn is_move_valid(&self, _scene: SceneId, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) -> bool { false }
}

//
// STATIC VARIABLES ARE NOT SHARED WITH DYNAMIC LIBRARIES!!!
//