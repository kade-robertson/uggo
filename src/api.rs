use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde_json::{Map, Number, Value};
use std::collections::HashMap;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

fn get_data(url: String) -> Option<Value> {
    match CLIENT.get(url).send() {
        Ok(response) => {
            if response.status().is_success() {
                let json_data = response.json::<Value>();
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
    let versions =
        get_data("https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json".to_string());
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

pub fn get_champ_data(version: &String) -> Option<Map<String, Value>> {
    let champ_data = get_data(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/champion.json",
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
    let rune_data = get_data(format!(
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
