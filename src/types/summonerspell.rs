use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummonerSpells {
    #[serde(rename = "type")]
    pub summoner_spell_data_type: String,
    pub version: String,
    pub data: HashMap<String, SummonerSpellDatum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummonerSpellDatum {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub maxrank: i64,
    pub cooldown: Vec<i64>,
    #[serde(rename = "cooldownBurn")]
    pub cooldown_burn: String,
    pub cost: Vec<i64>,
    #[serde(rename = "costBurn")]
    pub cost_burn: String,
    pub effect: Vec<Option<Vec<f64>>>,
    #[serde(rename = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    pub vars: Vec<Option<serde_json::Value>>,
    pub key: String,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: i64,
    pub modes: Vec<String>,
    #[serde(rename = "costType")]
    pub cost_type: CostType,
    pub maxammo: String,
    pub range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    pub range_burn: String,
    pub image: Image,
    pub resource: CostType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: Sprite,
    pub group: Group,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CostType {
    #[serde(rename = "&nbsp;")]
    Nbsp,
    #[serde(rename = "No Cost")]
    NoCost,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Group {
    #[serde(rename = "spell")]
    Spell,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "spell0.png")]
    Spell0Png,
}
