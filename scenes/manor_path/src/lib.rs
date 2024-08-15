use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
use assets::save_system::save_system::SaveSystem;
use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

/// Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![
            "ManorPath".to_string(),
            "manorPath".to_string(),
            "manorpath".to_string(),
            "MANORPATH".to_string(),
            "Manor Path".to_string(),
            "manor Path".to_string(),
            "manor path".to_string(),
            "MANOR PATH".to_string()
        ],
        id: SceneId::ManorPath
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(ManorPath)
}

pub struct ManorPath;

impl Scene for ManorPath {
    fn enter_scene(&self, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) {
        println!("By gaining access to the manor, you are able to solve the paranormal mystery!");
        _event_system.invoke(EventType::WinGame);
    }
}
