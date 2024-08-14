use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
use assets::inventory_system::items::ItemId;
use assets::save_system;
use assets::save_system::save_system::SaveSystem;
use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

/// Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![
            "ManorGate".to_string(),
            "manorGate".to_string(),
            "manorgate".to_string(),
            "MANORGATE".to_string()
        ],
        id: SceneId::ManorGate
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(ManorGate)
}

pub struct ManorGate;

impl Scene for ManorGate {
    fn enter_scene(&self, _event_system: &mut EventSystem, _save_system: &mut SaveSystem) {
        if _save_system.get_flag(&String::from("is_gargoyle_broke")) {
            self.display_room_info(_event_system, _save_system);
        } else {
            println!(concat!("You are the worlds best detective. Paranormal detective, that is. You have been called here, to the Dalmur Manor to investigate tales of ghosts.",
            "\nAre these just wives tales, or something more spectral? Either way, you'll get to the bottom of it. You're a professional.",
            "\nEverything always goes according to how you planned it. It's nightime with a full moon, perfect for the supernatural.",
            "\nUnfortunantly you forgot to plan for the weather. Not only can you not see the moon due to the clouds, but you also can't quite keep your eyes open while looking up.",
            "\nIt is raining. Hard. If you didn't have your trademarked trenchcoat you would be instantly soaked. Hopefully after you get inside the manor you'll have some respite.",
            "\nYou'll have to get past the GATE though. A tall, metal barred gate lies in front of you. It seems to have weathered the century it's stood there well.",
            "\nNo amount of brute force will get you through this thing. When you turn to investigate the walls, a figure leaps out at you from the nearby BUSH!",
            "\nThe gray cracked skin pulled roughly over its maw grinns at you while the thunder rolls off a boom for the creater. It startled you, but you have no need to be worried.",
            "\nIt is a GARGOYLE, a simple stone statue. Aha! Your plan was right again. There is a KEY BOX on the wall next to the gate.",
            "\nMost likely whoever takes care of abandoned buildings uses it to get the key to the gate. Your plan is all coming together. Now to figure out how to open that box up."));
    
        }
    }

    fn display_room_info(&self, _event_system: &mut EventSystem, _save_system: &mut assets::save_system::save_system::SaveSystem) {
        println!("{} Behind a nearby BUSH, {} and {}", 
            if _save_system.get_flag(&String::from("is_gate_open")) {
                "The large GATE leans slightly adjar, giving access to the MANOR PATH."
            } else {
                "A large GATE looms above you, blocking your path towards the manor."
            },
            if _save_system.get_flag(&String::from("is_gargoyle_broke")) {
                "there is a deformed GARGOYLE"
            } else {
                "you find a GARGOYLE"
            },
            if _save_system.get_flag(&String::from("is_key_box_open")) {
                "an open KEY BOX."
            } else {
                "a KEY BOX."
            }
        )
    }

    fn examine(&self, _examinable: &String,_event_system: &mut EventSystem, _save_system: &mut SaveSystem) {
        if _examinable == "gargoyle" {
            examine_gargoyle(_save_system);
        } else if _examinable == "bush" {
            examine_bush(_save_system);
        } else if _examinable == "key box" {
            examine_key_box(_save_system);
        } else if _examinable == "gate" {
            examine_gate(_save_system);
        } else {
            println!("{} is not examinable.", _examinable);
        }
    }

}


fn examine_gargoyle(save_system: &mut SaveSystem) {
    if save_system.get_flag(&String::from("is_gargoyle_broke")) {
        if save_system.get_flag(&String::from("is_math_clue_taken")) {
            println!("Its mouth is broken, making it considerably less scary.");
        } else {
            println!("Its mouth is broken, revealing a MATH CLUE hiding inside of it.");
        }
    } else {
        println!("It looks like there's a piece of paper stuck deep inside of its mouth.");
    }
}

fn examine_bush(save_system: &mut SaveSystem) {
    if save_system.get_flag(&String::from("is_chisel_taken")) {
        println!("It's a plant. Nothing useful.");
    } else {
        println!("It's a plant. There's a lot of junk caught underneath it next to the wall. Some broken glass, an old oil lantern, and a CHISEL.");
    }
}

fn examine_key_box(save_system: &mut SaveSystem) {
    if save_system.get_flag(&String::from("is_box_open")) {
        if save_system.get_flag(&String::from("is_key_taken")) {
            println!("An empty open box.");
        } else {
            println!("An open box with a lonesome key inside.");
        }
    } else {
        println!("A box with a 4-letter combination lock. It most likely has the key to the gate inside.");
    }
}

fn examine_gate(save_system: &mut SaveSystem) {
    if save_system.get_flag(&String::from("is_gate_open")) {
        println!("A large gate leans slightly adjar, daring you to enter the manor property.");
    } else {
        println!("A large gate that seperates the manor property from the long road up the hill.");
    }
}