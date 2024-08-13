use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
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
            "MANORPATH".to_string()
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
    fn enter_scene(&self, _event_system: &mut EventSystem) {
        println!("The Dark Forest stands before you. You need to go through it to get to Grandma's house. 
        The forest is notorious for disappearances. It's easy to get lost in it's vast labyranth. 
        Entering the forest, the path immediatly branches.");
        println!("To the LEFT is the sound of bubbling water. To the RIGHT you can barely make out small points of light.");
    }
}
