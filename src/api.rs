use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde_json::Value;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

fn get_data(url: &str) -> Option<Value> {
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
    let versions = get_data("https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json");
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
