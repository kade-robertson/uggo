use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type RunePages = Vec<RunePage>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunePage {
    pub auto_modified_selections: Vec<Value>,
    pub current: bool,
    pub id: i64,
    pub is_active: bool,
    pub is_deletable: bool,
    pub is_editable: bool,
    pub is_valid: bool,
    pub last_modified: i64,
    pub name: String,
    pub order: i64,
    pub primary_style_id: i64,
    pub selected_perk_ids: Vec<i64>,
    pub sub_style_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRunePage {
    pub name: String,
    pub primary_style_id: i64,
    pub selected_perk_ids: Vec<i64>,
    pub sub_style_id: i64,
}
