use phf::phf_map;
use serde::{Deserialize, Serialize};

///An ID for items
#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum ItemId {
    Chisel,
    MathClue,
    GateKey
}

impl ItemId {
    pub fn to_string(&self) -> &'static str {
        match self {
            ItemId::Chisel => "Chisel",
            ItemId::MathClue => "Math Clue",
            ItemId::GateKey => "Gate Key"
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            ItemId::Chisel => "A simple mason chisel made for cutting stone.",
            ItemId::GateKey => "A golden skeleton key with a caligraphic D as the head. A matching symbol is also the centralpiece decoration of the gate.",
            ItemId::MathClue => "A small scrap of paper that reads \"2 + 2 = \" in poor handrighting. Whoever wrote it didn't have very good education, one of the numbers is backwards."
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
    "MATHCLUE" => ItemId::MathClue,
    "Gate Key" => ItemId::GateKey,
    "Gate key" => ItemId::GateKey,
    "gate Key" => ItemId::GateKey,
    "GATE KEY" => ItemId::GateKey,
    "GateKey" => ItemId::GateKey,
    "Gatekey" => ItemId::GateKey,
    "gateKey" => ItemId::GateKey,
    "GATEKEY" => ItemId::GateKey
};