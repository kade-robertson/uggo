use crate::types::champion::ChampionData;
use crate::{mappings, types::champion::Datum};
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde_json::{Map, Number, Value};
use std::{collections::HashMap, convert::TryFrom};

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

fn get_data<T: DeserializeOwned>(url: String) -> Option<T> {
    match CLIENT.get(url).send() {
        Ok(response) => {
            if response.status().is_success() {
                let json_data = response.json::<T>();
                match json_data {
                    Ok(json) => {
                        return Some(json);
                    }
                    Err(_) => {
                        return None;
                    }
                }
            } else {
                return None;
            }
        }
        Err(_) => {
            return None;
        }
    }
}

pub fn get_current_version() -> Option<String> {
    let versions = get_data::<Value>(
        "https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json".to_string(),
    );
    match versions {
        Some(vers) => {
            if vers.is_array() && vers[0].is_string() {
                return Some(String::from(vers[0].as_str().unwrap()));
            } else {
                return None;
            }
        }
        None => {
            return None;
        }
    }
}

pub fn get_champ_data(version: &String) -> Option<Box<HashMap<String, Datum>>> {
    let champ_data = get_data::<ChampionData>(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/champion.json",
        version
    ));
    match champ_data {
        Some(data) => {
            return Some(Box::new(data.data));
        }
        None => {
            return None;
        }
    }
}

pub fn get_items(version: &String) -> Option<Map<String, Value>> {
    let champ_data = get_data::<Value>(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/item.json",
        version
    ));
    match champ_data {
        Some(data) => {
            if data.is_object() && data.as_object().unwrap().contains_key("data") {
                let unwrapped_data = data.as_object().unwrap();
                if unwrapped_data.contains_key("data") && unwrapped_data["data"].is_object() {
                    return Some(unwrapped_data["data"].as_object().unwrap().clone());
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        None => {
            return None;
        }
    }
}

pub fn get_runes(version: &String) -> Option<HashMap<i64, Map<String, Value>>> {
    let rune_data = get_data::<Value>(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/runesReforged.json",
        version
    ));
    match rune_data {
        Some(data) => {
            if data.is_array() && data.as_array().unwrap().len() > 0 {
                let unwrapped_data = data.as_array().unwrap();
                let mut processed_data = HashMap::new();
                for class in unwrapped_data {
                    if class.is_object() && class.as_object().unwrap().contains_key("slots") {
                        let unwrapped_class = class.as_object().unwrap();
                        let rune_slots = &unwrapped_class["slots"];
                        for (slot_index, slot) in rune_slots.as_array().unwrap().iter().enumerate()
                        {
                            let runes = &slot.as_object().unwrap()["runes"];
                            for (index, rune) in runes.as_array().unwrap().iter().enumerate() {
                                let mut cloned_rune = rune.clone();
                                let unwrapped_rune = cloned_rune.as_object_mut().unwrap();
                                unwrapped_rune.insert(
                                    "slot".to_string(),
                                    Value::Number(Number::from(slot_index)),
                                );
                                unwrapped_rune.insert(
                                    "index".to_string(),
                                    Value::Number(Number::from(index)),
                                );
                                unwrapped_rune.insert(
                                    "parent".to_string(),
                                    Value::String(
                                        unwrapped_class["name"].as_str().unwrap().to_string(),
                                    ),
                                );
                                processed_data.insert(
                                    unwrapped_rune["id"].as_i64().unwrap(),
                                    unwrapped_rune.clone(),
                                );
                            }
                        }
                    }
                }
                return Some(processed_data);
            } else {
                return None;
            }
        }
        None => {
            return None;
        }
    }
}

pub fn get_summoner_spells(version: &String) -> Option<HashMap<i64, String>> {
    let summoner_data = get_data::<Value>(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/summoner.json",
        version
    ));
    match summoner_data {
        Some(spells) => {
            if spells.is_object() && spells.as_object().unwrap().contains_key("data") {
                let spell_data = spells.as_object().unwrap()["data"].as_object().unwrap();
                let mut reduced_data: HashMap<i64, String> = HashMap::new();
                for (_spell, spell_info) in spell_data {
                    reduced_data.insert(
                        spell_info["key"].as_str().unwrap().parse::<i64>().unwrap(),
                        spell_info["name"].as_str().unwrap().to_string(),
                    );
                }
                return Some(reduced_data);
            } else {
                return None;
            }
        }
        None => {
            return None;
        }
    }
}

pub fn get_stats(
    patch: &str,
    champ: &Datum,
    role: mappings::Role,
    region: mappings::Region,
    mode: mappings::Mode,
) -> Option<(mappings::Role, Vec<Value>)> {
    let stats_data = get_data::<Value>(format!(
        "https://stats2.u.gg/lol/1.1/overview/{}/{}/{}/1.4.0.json",
        patch,
        mode.to_string(),
        champ.key.as_str()
    ));
    match stats_data {
        Some(champ_stats) => {
            if champ_stats.is_object() {
                let unwrapped_stats = champ_stats.as_object().unwrap();
                let stats_for_region = unwrapped_stats[&(region as i32).to_string()]
                    .as_object()
                    .unwrap();
                let rank_query = if stats_for_region
                    .contains_key(&mappings::rank_to_str(mappings::Rank::PlatinumPlus))
                {
                    mappings::Rank::PlatinumPlus
                } else {
                    mappings::Rank::Overall
                };
                let stats_for_rank = stats_for_region[&mappings::rank_to_str(rank_query)]
                    .as_object()
                    .unwrap();
                let mut role_query = role;
                if !stats_for_rank.contains_key(&mappings::role_to_str(role_query)) {
                    if role_query == mappings::Role::Automatic {
                        // Go through each role and pick the one with most matches played
                        let mut most_games = 0;
                        let mut used_role = stats_for_rank.keys().next().unwrap();
                        for (role_key, role_stats) in stats_for_rank {
                            let games_played = role_stats[0][6][1].as_i64().unwrap();
                            if games_played > most_games {
                                most_games = games_played;
                                used_role = role_key;
                            }
                        }
                        role_query =
                            mappings::Role::try_from(used_role.parse::<i32>().unwrap()).unwrap();
                    } else {
                        // This should only happen in ARAM
                        role_query = mappings::Role::None;
                    }
                }
                return Some((
                    role_query,
                    stats_for_rank[&mappings::role_to_str(role_query)]
                        .as_array()
                        .unwrap()
                        .clone(),
                ));
            } else {
                return None;
            }
        }
        None => {
            return None;
        }
    }
}
