use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde_json::{Map, Value};

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

pub fn get_champ_data(version: String) -> Option<Map<String, Value>> {
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
