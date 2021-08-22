use levenshtein::levenshtein;
use serde_json::{Map, Value};

pub fn find_champ<'a>(name: &str, champ_data: &'a Map<String, Value>) -> &'a Value {
    if champ_data.contains_key(name) {
        return &champ_data[name];
    } else {
        let mut lowest_distance: i32 = i32::MAX;
        let mut closest_champ: &Value = &champ_data[champ_data.keys().next().unwrap()];
        for (key, value) in champ_data {
            let distance = levenshtein(name, key) as i32;
            if distance < lowest_distance {
                lowest_distance = distance;
                closest_champ = value;
            }
        }
        return closest_champ;
    }
}
