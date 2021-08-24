use serde::{Deserialize, Serialize};

pub type RunePaths = Vec<RunePath>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunePath {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<Slot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    pub runes: Vec<Rune>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rune {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuneExtended {
    pub slot: i64,
    pub index: i64,
    pub parent: String,
    pub rune: Rune,
}
