use colored::Colorize;
use ddragon::models::champions::ChampionShort;
use ddragon::models::items::Item;
use ddragon::models::runes::RuneElement;
use std::collections::HashMap;

#[cfg(debug_assertions)]
use time::{macros::format_description, OffsetDateTime};

use ugg_types::rune::RuneExtended;

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
    champ_data: &'_ HashMap<String, ChampionShort>,
) -> Option<&'_ ChampionShort> {
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
    rune_data: &'a HashMap<i64, RuneExtended<RuneElement>>,
) -> Vec<(String, Vec<&'a RuneExtended<RuneElement>>)> {
    let mut grouped_runes: Vec<(String, Vec<&'a RuneExtended<RuneElement>>)> = Vec::new();

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

pub fn process_items(champ_items: &[i64], item_data: &HashMap<String, Item>) -> String {
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

pub fn generate_perk_array(
    runes: &[(String, Vec<&RuneExtended<RuneElement>>)],
    shards: &[i64],
) -> (i64, i64, Vec<i64>) {
    let mut perk_list: Vec<i64> = Vec::new();
    perk_list.append(&mut runes[0].1.iter().map(|el| el.rune.id).collect::<Vec<i64>>());
    perk_list.append(&mut runes[1].1.iter().map(|el| el.rune.id).collect::<Vec<i64>>());
    perk_list.append(&mut shards.to_vec());
    (runes[0].1[0].parent_id, runes[1].1[0].parent_id, perk_list)
}
