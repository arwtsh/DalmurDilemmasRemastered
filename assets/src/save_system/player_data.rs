use log::{info, warn};
use serde::{Deserialize, Serialize};
use crate::{inventory_system::items::ItemId, scene_system::scene_id::{SceneId, STARTING_SCENE}};
use std::collections::{hash_set::Iter, HashSet};

use super::{profile_data::ProfileData, save_system::SaveSystem};

pub(super) const DEFAULT_PROFILE_PLAYER: PlayerData = PlayerData { //The player portion of the default profile data.
    current_scene: STARTING_SCENE,
    inventory: Option::None
};

/// All of the save data that represents the current player state
/// For example: the current scene the player is in, not counting a main menu or pause menu.
#[derive(Deserialize, Serialize)]
pub(super)  struct PlayerData {
    //The current "gameplay" scene the player is in. 
    //This does not count scenes like the main menu. 
    //This variable is not referenced during runtime, only by loading saves.
    pub current_scene: SceneId,
    //The current inventory of the player.
    //If the item is in this set, the player has it.
    pub inventory: Option<HashSet<ItemId>>
}

impl ProfileData {
    pub(super) fn get_inventory(&mut self) -> &mut HashSet<ItemId> {
        self.player_data.inventory.get_or_insert_with(|| HashSet::new())
    }
}

impl SaveSystem {
    /// Gets the current scene the player is in.
    /// This only counts gameplay scenes, not the Main Menu for example.
    pub fn get_current_scene(&self) -> SceneId {
        self.get_profile().player_data.current_scene
    }

    /// Sets the current scene the player is in.
    /// If the scene is not marked as saveable, nothing happens.
    pub fn set_current_scene(&mut self, new_scene: SceneId) {
        //Only update if the scene is saveable.
        if new_scene.is_saveable() {
            self.get_mut_profile().player_data.current_scene = new_scene;
            //self.get_mut_profile_wrapper().has_changed = true;
            self.save_profile();
        }
    }

    /// Remove item from the inventory
    pub fn lose_item(&mut self, item: &ItemId) {
        if self.get_mut_profile().get_inventory().remove(item) {
            info!("Removed item {} from player's inventory.", item.to_string());
            self.save_profile();
        } else {
            warn!("Tried to remove item {} from player's inventory, but it was absent.", item.to_string());
        }
    }

    pub fn add_item(&mut self, item: &ItemId) {
        if self.get_mut_profile().get_inventory().insert(item.clone()) {
            info!("Added item {} to player's inventory.", item.to_string());
            self.save_profile();
        } else {
            warn!("Tried to add item {} to player's inventory, but it was already in there.", item.to_string());
        }
    }

    pub fn is_item_in_inventory(&mut self, item: &ItemId) -> bool {
        self.get_mut_profile().get_inventory().contains(item)
    }

    /// Get an iterator of all items in the player's inventory.
    pub fn get_inventory_iter(&mut self) -> Iter<'_, ItemId> {
        self.get_mut_profile().get_inventory().iter()
    }

    pub fn clear_inventory(&mut self) {
        info!("Cleared player's inventory.");
        self.get_mut_profile().get_inventory().clear();
        self.save_profile();
    }
}