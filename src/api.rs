use crate::mappings;
use crate::types::champion::{ChampionDatum, Champions};
use crate::types::item::{ItemDatum, Items};
use crate::types::matchups::{MatchupData, Matchups};
use crate::types::overview::{ChampOverview, OverviewData};
use crate::types::rune::{RuneExtended, RunePaths};
use crate::types::summonerspell::SummonerSpells;
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

fn get_data<T: DeserializeOwned>(url: String) -> Option<T> {
    match CLIENT.get(url).send() {
        Ok(response) => {
            if response.status().is_success() {
                let json_data = response.json::<T>();
                match json_data {
                    Ok(json) => Some(json),
                    Err(_) => None,
                }
            } else {
                return None;
            }
        }
        Err(_) => None,
    }
}

pub fn get_current_version() -> Option<String> {
    let versions = get_data::<Vec<String>>(
        "https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json".to_string(),
    );
    match versions {
        Some(vers) => Some(vers[0].as_str().to_string()),
        None => None,
    }
}

pub fn get_champ_data(version: &String) -> Option<Box<HashMap<String, ChampionDatum>>> {
    let champ_data = get_data::<Champions>(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/champion.json",
        version
    ));
    match champ_data {
        Some(data) => Some(Box::new(data.data)),
        None => None,
    }
}

pub fn get_items(version: &String) -> Option<Box<HashMap<String, ItemDatum>>> {
    let champ_data = get_data::<Items>(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/item.json",
        version
    ));
    match champ_data {
        Some(data) => Some(Box::new(data.data)),
        None => None,
    }
}

pub fn get_runes(version: &String) -> Option<Box<HashMap<i64, RuneExtended>>> {
    let rune_data = get_data::<RunePaths>(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/runesReforged.json",
        version
    ));
    match rune_data {
        Some(data) => {
            let mut processed_data = HashMap::new();
            for class in data {
                for (slot_index, slot) in class.slots.iter().enumerate() {
                    for (index, rune) in slot.runes.iter().enumerate() {
                        let extended_rune = RuneExtended {
                            rune: (*rune).clone(),
                            slot: slot_index as i64,
                            index: index as i64,
                            siblings: slot.runes.len() as i64,
                            parent: class.name.clone(),
                        };
                        processed_data.insert(rune.id, extended_rune);
                    }
                }
            }
            return Some(Box::new(processed_data));
        }
        None => None,
    }
}

pub fn get_summoner_spells(version: &String) -> Option<Box<HashMap<i64, String>>> {
    let summoner_data = get_data::<SummonerSpells>(format!(
        "https://static.u.gg/assets/lol/riot_static/{}/data/en_US/summoner.json",
        version
    ));
    match summoner_data {
        Some(spells) => {
            let mut reduced_data: HashMap<i64, String> = HashMap::new();
            for (_spell, spell_info) in spells.data {
                reduced_data.insert(
                    spell_info.key.parse::<i64>().ok().unwrap_or(0),
                    spell_info.name,
                );
            }
            return Some(Box::new(reduced_data));
        }
        None => None,
    }
}

pub fn get_stats(
    patch: &str,
    champ: &ChampionDatum,
    role: mappings::Role,
    region: mappings::Region,
    mode: mappings::Mode,
) -> Option<Box<(mappings::Role, OverviewData)>> {
    let stats_data = get_data::<ChampOverview>(format!(
        "https://stats2.u.gg/lol/1.1/overview/{}/{}/{}/1.4.0.json",
        patch,
        mode.to_string(),
        champ.key.as_str()
    ));
    match stats_data {
        Some(champ_stats) => {
            let region_query = if champ_stats.contains_key(&region) {
                region
            } else {
                mappings::Region::World
            };

            let rank_query =
                if champ_stats[&region_query].contains_key(&mappings::Rank::PlatinumPlus) {
                    mappings::Rank::PlatinumPlus
                } else {
                    mappings::Rank::Overall
                };

            let mut role_query = role;
            if !champ_stats[&region_query][&rank_query].contains_key(&role_query) {
                if role_query == mappings::Role::Automatic {
                    // Go through each role and pick the one with most matches played
                    let mut most_games = 0;
                    let mut used_role = role;
                    for (role_key, role_stats) in &champ_stats[&region_query][&rank_query] {
                        if role_stats.data.matches > most_games {
                            most_games = role_stats.data.matches;
                            used_role = role_key.clone();
                        }
                    }
                    role_query = used_role;
                } else {
                    // This should only happen in ARAM
                    role_query = mappings::Role::None;
                }
            }
            return Some(Box::new((
                role_query,
                champ_stats[&region_query][&rank_query][&role_query]
                    .data
                    .clone(),
            )));
        }
        None => None,
    }
}

pub fn get_matchups(
    patch: &str,
    champ: &ChampionDatum,
    role: mappings::Role,
    region: mappings::Region,
    mode: mappings::Mode,
) -> Option<Box<(mappings::Role, MatchupData)>> {
    let stats_data = get_data::<Matchups>(format!(
        "https://stats2.u.gg/lol/1.1/matchups/{}/{}/{}/1.4.0.json",
        patch,
        mode.to_string(),
        champ.key.as_str()
    ));
    match stats_data {
        Some(champ_stats) => {
            let region_query = if champ_stats.contains_key(&region) {
                region
            } else {
                mappings::Region::World
            };

            let rank_query =
                if champ_stats[&region_query].contains_key(&mappings::Rank::PlatinumPlus) {
                    mappings::Rank::PlatinumPlus
                } else {
                    mappings::Rank::Overall
                };

            let mut role_query = role;
            if !champ_stats[&region_query][&rank_query].contains_key(&role_query) {
                if role_query == mappings::Role::Automatic {
                    // Go through each role and pick the one with most matches played
                    //let mut most_games = 0;
                    //let mut used_role = role;
                    /*for (role_key, role_stats) in &champ_stats[&region_query][&rank_query] {
                        if role_stats.data.total_matches > most_games {
                            most_games = role_stats.data.total_matches;
                            used_role = role_key.clone();
                        }
                    }*/
                    role_query = mappings::Role::Top;
                } else {
                    // This should only happen in ARAM
                    role_query = mappings::Role::None;
                }
            }
            return Some(Box::new((
                role_query,
                champ_stats[&region_query][&rank_query][&role_query]
                    .data
                    .clone(),
            )));
        }
        None => None,
    }
}
