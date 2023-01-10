use colored::Colorize;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::{collections::HashMap, fs};

#[cfg(debug_assertions)]
use time::{macros::format_description, OffsetDateTime};

use crate::types::{champion::ChampionDatum, item::ItemDatum, rune::RuneExtended};

#[cfg(debug_assertions)]
static TIME_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");

#[cfg(debug_assertions)]
fn now_fmt() -> String {
    OffsetDateTime::now_local()
        .map_or_else(|_| OffsetDateTime::now_utc(), |t| t)
        .format(TIME_FORMAT)
        .unwrap()
}

pub fn log_error(msg: &str) {
    #[cfg(debug_assertions)]
    {
        eprint!("[{}] ", now_fmt().as_str());
    }
    eprint!("{} ", "Error:".red().bold());
    eprintln!("{msg}");
}

pub fn log_info(msg: &str) {
    let mut message = String::new();
    #[cfg(debug_assertions)]
    {
        message.push_str(format!("[{}] ", now_fmt()).as_str());
    }
    message.push_str(msg);
    println!("{message}");
}

pub fn find_champ_by_key(
    key: i64,
    champ_data: &'_ HashMap<String, ChampionDatum>,
) -> Option<&'_ ChampionDatum> {
    match champ_data
        .iter()
        .find(|champ| champ.1.key == key.to_string())
    {
        Some(data) => Some(data.1),
        None => None,
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

    grouped_runes
}

pub fn process_items(champ_items: &[i64], item_data: &HashMap<String, ItemDatum>) -> String {
    champ_items
        .iter()
        .map(|v| {
            item_data
                .get(&v.to_string())
                .map_or_else(|| format!("<unknown item {v}>"), |i| i.name.clone())
        })
        .collect::<Vec<String>>()
        .join(", ")
}

const fn get_shard(id: &i64) -> &str {
    match id {
        5001 => "+15-90 Health",
        5002 => "+6 Armor",
        5003 => "+8 Magic Resist",
        5005 => "+10% Attack Speed",
        5007 => "+8 Ability Haste",
        5008 => "+9 Adaptive Force",
        _ => "Unknown",
    }
}

pub fn process_shards(shards: &[i64]) -> Vec<String> {
    let mut shard_text: Vec<String> = Vec::new();
    shard_text.push(format!("- Offense: {}", get_shard(&shards[0])));
    shard_text.push(format!("- Flex: {}", get_shard(&shards[1])));
    shard_text.push(format!("- Defense: {}", get_shard(&shards[2])));
    shard_text
}

pub fn sha256(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

pub fn read_from_cache<T: DeserializeOwned>(cache_dir: &str, filename: &str) -> Option<T> {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    if file_path.exists() {
        match serde_json::from_str::<T>(&fs::read_to_string(file_path).unwrap_or_default()) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub fn write_to_cache<T: Serialize>(cache_dir: &str, filename: &str, data: &T) {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    if let Ok(data) = serde_json::to_string::<T>(data) {
        fs::write(file_path, data).ok();
    }
}

pub fn clear_cache(cache_dir: &str, filename: &str) {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    if file_path.exists() {
        fs::remove_file(file_path).ok();
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
pub fn generate_perk_array(
    runes: &[(String, Vec<&RuneExtended>)],
    shards: &[i64],
) -> (i64, i64, Vec<i64>) {
    let mut perk_list: Vec<i64> = Vec::new();
    perk_list.append(&mut runes[0].1.iter().map(|el| el.rune.id).collect::<Vec<i64>>());
    perk_list.append(&mut runes[1].1.iter().map(|el| el.rune.id).collect::<Vec<i64>>());
    perk_list.append(&mut shards.to_vec());
    (runes[0].1[0].parent_id, runes[1].1[0].parent_id, perk_list)
}
