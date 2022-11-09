use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Champions {
    #[serde(rename = "type")]
    pub welcome_type: Type,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampionDatum>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChampionDatum {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: Info,
    pub image: Image,
    pub tags: Vec<Tag>,
    pub partype: String,
    pub stats: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub full: String,
    pub sprite: Sprite,
    pub group: Type,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Info {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum Type {
    #[serde(rename = "champion")]
    Champion,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sprite {
    #[serde(rename = "champion0.png")]
    Champion0,
    #[serde(rename = "champion1.png")]
    Champion1,
    #[serde(rename = "champion2.png")]
    Champion2,
    #[serde(rename = "champion3.png")]
    Champion3,
    #[serde(rename = "champion4.png")]
    Champion4,
    #[serde(rename = "champion5.png")]
    Champion5,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tag {
    Assassin,
    Fighter,
    Mage,
    Marksman,
    Support,
    Tank,
}
