use crate::mappings;
use serde_json::Value;
use std::collections::HashMap;

pub type ChampOverview =
    HashMap<mappings::Region, HashMap<mappings::Rank, HashMap<mappings::Role, Vec<Value>>>>;
