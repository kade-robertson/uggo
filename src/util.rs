use levenshtein::levenshtein;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn find_champ<'a>(name: &str, champ_data: &'a Map<String, Value>) -> &'a Value {
    if champ_data.contains_key(name) {
        return &champ_data[name];
    } else {
        let mut lowest_distance: i32 = i32::MAX;
        let mut closest_champ: &Value = &champ_data["Annie"];
        for (_key, value) in champ_data {
            let distance = levenshtein(name, value["name"].as_str().unwrap()) as i32;
            if distance < lowest_distance {
                lowest_distance = distance;
                closest_champ = value;
            }
        }
        return closest_champ;
    }
}

pub fn group_runes<'a>(
    champ_runes: &Vec<Value>,
    rune_data: &'a HashMap<i64, Map<String, Value>>,
) -> Vec<(String, Vec<&'a Map<String, Value>>)> {
    let mut grouped_runes: Vec<(String, Vec<&'a Map<String, Value>>)> = Vec::new();

    for rune in champ_runes {
        let rune_info = &rune_data[&rune.as_i64().unwrap()];
        let group_name = rune_info["parent"].as_str().unwrap();
        let group_index = grouped_runes.iter().position(|r| r.0 == group_name);
        if group_index.is_none() {
            grouped_runes.push((group_name.to_string(), vec![rune_info]));
        } else {
            grouped_runes[group_index.unwrap()].1.push(rune_info);
        }
    }

    // Make sure primary rune is first
    if grouped_runes[0].1.len() != 4 {
        grouped_runes.reverse();
    }

    return grouped_runes;
}

pub fn process_items(champ_items: &Value, item_data: &Map<String, Value>, nested: bool) -> String {
    return champ_items
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            item_data[&(if nested { &v[0] } else { v })
                .as_i64()
                .unwrap()
                .to_string()]["name"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join(", ");
}
