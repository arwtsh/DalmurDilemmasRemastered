use phf::phf_map;
use serde::{Deserialize, Serialize};

///An ID for items
#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum ItemId {
    Chisel,
    MathClue
}

impl ItemId {
    pub fn to_string(&self) -> &'static str {
        match self {
            ItemId::Chisel => "Chisel",
            ItemId::MathClue => "MathClue"
        }
    }
}

pub fn parse_item(name: &String) -> Option<&ItemId> {
    ITEM_PARSES.get(name)
}

pub(super) const ITEM_PARSES: phf::Map<&'static str, ItemId> = phf_map! {
    "chisel" => ItemId::Chisel,
    "CHISEL" => ItemId::Chisel,
    "Chisel" => ItemId::Chisel,
    "Math Clue" => ItemId::MathClue,
    "math Clue" => ItemId::MathClue,
    "Math clue" => ItemId::MathClue,
    "math clue" => ItemId::MathClue,
    "MATH CLUE" => ItemId::MathClue,
    "MathClue" => ItemId::MathClue,
    "mathClue" => ItemId::MathClue,
    "Mathclue" => ItemId::MathClue,
    "mathclue" => ItemId::MathClue,
    "MATHCLUE" => ItemId::MathClue
};