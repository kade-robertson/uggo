#[cfg(debug_assertions)]
use chrono;
use colored::*;
use levenshtein::levenshtein;
use std::collections::HashMap;

use crate::types::{champion::ChampionDatum, item::ItemDatum, rune::RuneExtended};

#[cfg(debug_assertions)]
static TIME_FORMAT: &str = "[%Y-%m-%d %H:%M:%S%.3f] ";

pub fn log_error(msg: &str) {
    #[cfg(debug_assertions)]
    {
        let now = chrono::Local::now();
        eprint!("{}", now.format(TIME_FORMAT).to_string().as_str());
    }
    eprint!("{} ", "Error:".red().bold());
    eprintln!("{}", msg);
}

pub fn log_info(msg: &str) {
    let mut message = String::new();
    #[cfg(debug_assertions)]
    {
        let now = chrono::Local::now();
        message.push_str(now.format(TIME_FORMAT).to_string().as_str());
    }
    message.push_str(msg);
    println!("{}", message);
}

pub fn find_champ<'a>(
    name: &str,
    champ_data: &'a HashMap<String, ChampionDatum>,
) -> &'a ChampionDatum {
    if champ_data.contains_key(name) {
        return &champ_data[name];
    } else {
        let mut lowest_distance: i32 = i32::MAX;
        let mut closest_champ: &ChampionDatum = &champ_data["Annie"];
        for (_key, value) in champ_data {
            let distance = levenshtein(name, value.name.as_str()) as i32;
            if distance < lowest_distance {
                lowest_distance = distance;
                closest_champ = value;
            }
        }
        return closest_champ;
    }
}

pub fn group_runes<'a>(
    rune_ids: &Vec<i64>,
    rune_data: &'a HashMap<i64, RuneExtended>,
) -> Vec<(String, Vec<&'a RuneExtended>)> {
    let mut grouped_runes: Vec<(String, Vec<&'a RuneExtended>)> = Vec::new();

    for rune in rune_ids {
        let rune_info = &rune_data[rune];
        match grouped_runes.iter().position(|r| r.0 == rune_info.parent) {
            Some(group_index) => grouped_runes[group_index].1.push(rune_info),
            None => grouped_runes.push((rune_info.parent.to_string(), vec![rune_info])),
        }
    }

    // Make sure primary rune is first
    if grouped_runes[0].1.len() != 4 {
        grouped_runes.reverse();
    }

    grouped_runes
        .iter_mut()
        .for_each(|group| group.1.sort_by(|&a, &b| a.slot.cmp(&b.slot)));

    return grouped_runes;
}

pub fn process_items(champ_items: &Vec<i64>, item_data: &HashMap<String, ItemDatum>) -> String {
    return champ_items
        .iter()
        .map(|v| item_data[&v.to_string()].name.clone())
        .collect::<Vec<String>>()
        .join(", ");
}

fn get_shard(id: &i64) -> &str {
    return match id {
        5001 => "+15-90 Health",
        5002 => "+6 Armor",
        5003 => "+8 Magic Resist",
        5005 => "+10% Attack Speed",
        5007 => "+8 Ability Haste",
        5008 => "+9 Adaptive Force",
        _ => "Unknown",
    };
}

pub fn process_shards(shards: &Vec<i64>) -> Vec<String> {
    let mut shard_text: Vec<String> = Vec::new();
    shard_text.push(format!("- Offense: {}", get_shard(&shards[0])));
    shard_text.push(format!("- Flex: {}", get_shard(&shards[1])));
    shard_text.push(format!("- Defense: {}", get_shard(&shards[2])));
    return shard_text;
}
