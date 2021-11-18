use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    #[serde(rename = "type")]
    pub item_data_type: Type,
    pub version: String,
    pub basic: Basic,
    pub data: HashMap<String, ItemDatum>,
    pub groups: Vec<Group>,
    pub tree: Vec<Tree>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Basic {
    pub name: String,
    pub rune: Rune,
    pub gold: Gold,
    pub group: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    pub consumed: bool,
    pub stacks: i64,
    pub depth: i64,
    #[serde(rename = "consumeOnFull")]
    pub consume_on_full: bool,
    pub from: Vec<Option<serde_json::Value>>,
    pub into: Vec<Option<serde_json::Value>>,
    #[serde(rename = "specialRecipe")]
    pub special_recipe: i64,
    #[serde(rename = "inStore")]
    pub in_store: bool,
    #[serde(rename = "hideFromAll")]
    pub hide_from_all: bool,
    #[serde(rename = "requiredChampion")]
    pub required_champion: String,
    #[serde(rename = "requiredAlly")]
    pub required_ally: String,
    pub stats: HashMap<String, i64>,
    pub tags: Vec<Option<serde_json::Value>>,
    pub maps: HashMap<String, bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gold {
    pub base: i64,
    pub total: i64,
    pub sell: i64,
    pub purchasable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rune {
    pub isrune: bool,
    pub tier: i64,
    #[serde(rename = "type")]
    pub rune_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDatum {
    pub name: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    pub into: Option<Vec<String>>,
    pub image: Image,
    pub gold: Gold,
    pub tags: Vec<String>,
    pub maps: HashMap<String, bool>,
    pub stats: HashMap<String, f64>,
    #[serde(rename = "inStore")]
    pub in_store: Option<bool>,
    pub from: Option<Vec<String>>,
    pub effect: Option<Effect>,
    pub depth: Option<i64>,
    pub stacks: Option<i64>,
    pub consumed: Option<bool>,
    #[serde(rename = "hideFromAll")]
    pub hide_from_all: Option<bool>,
    #[serde(rename = "consumeOnFull")]
    pub consume_on_full: Option<bool>,
    #[serde(rename = "requiredChampion")]
    pub required_champion: Option<String>,
    #[serde(rename = "specialRecipe")]
    pub special_recipe: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Effect {
    #[serde(rename = "Effect1Amount")]
    pub effect1_amount: String,
    #[serde(rename = "Effect2Amount")]
    pub effect2_amount: Option<String>,
    #[serde(rename = "Effect3Amount")]
    pub effect3_amount: Option<String>,
    #[serde(rename = "Effect4Amount")]
    pub effect4_amount: Option<String>,
    #[serde(rename = "Effect5Amount")]
    pub effect5_amount: Option<String>,
    #[serde(rename = "Effect6Amount")]
    pub effect6_amount: Option<String>,
    #[serde(rename = "Effect7Amount")]
    pub effect7_amount: Option<String>,
    #[serde(rename = "Effect8Amount")]
    pub effect8_amount: Option<String>,
    #[serde(rename = "Effect9Amount")]
    pub effect9_amount: Option<String>,
    #[serde(rename = "Effect10Amount")]
    pub effect10_amount: Option<String>,
    #[serde(rename = "Effect11Amount")]
    pub effect11_amount: Option<String>,
    #[serde(rename = "Effect12Amount")]
    pub effect12_amount: Option<String>,
    #[serde(rename = "Effect13Amount")]
    pub effect13_amount: Option<String>,
    #[serde(rename = "Effect14Amount")]
    pub effect14_amount: Option<String>,
    #[serde(rename = "Effect15Amount")]
    pub effect15_amount: Option<String>,
    #[serde(rename = "Effect16Amount")]
    pub effect16_amount: Option<String>,
    #[serde(rename = "Effect17Amount")]
    pub effect17_amount: Option<String>,
    #[serde(rename = "Effect18Amount")]
    pub effect18_amount: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: Sprite,
    pub group: Type,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    #[serde(rename = "MaxGroupOwnable")]
    pub max_group_ownable: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub header: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "item")]
    Item,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "item0.png")]
    Item0Png,
    #[serde(rename = "item1.png")]
    Item1Png,
    #[serde(rename = "item2.png")]
    Item2Png,
}
