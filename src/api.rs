use crate::config::Config;
use crate::mappings;
use crate::types::champion::{ChampionDatum, Champions};
use crate::types::item::{ItemDatum, Items};
use crate::types::matchups::{MatchupData, Matchups};
use crate::types::overview::{ChampOverview, OverviewData};
use crate::types::rune::{RuneExtended, RunePaths};
use crate::types::summonerspell::SummonerSpells;
use crate::util::{clear_cache, read_from_cache, sha256, write_to_cache};
use lru::LruCache;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

type UggAPIVersions = HashMap<String, HashMap<String, String>>;

pub struct API {
    _client: Client,
    _config: Config,
    _overview_lru_cache: LruCache<String, ChampOverview>,
    _matchup_lru_cache: LruCache<String, Matchups>,
}

impl API {
    pub fn new() -> API {
        API {
            _client: Client::new(),
            _config: Config::new(),
            _overview_lru_cache: LruCache::new(25),
            _matchup_lru_cache: LruCache::new(25),
        }
    }

    fn get_data<T: DeserializeOwned>(&self, url: &String) -> Option<T> {
        match self._client.get(url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    let json_data = response.json::<T>();
                    match json_data {
                        Ok(json) => Some(json),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn get_cached_data<T: DeserializeOwned + Serialize>(&self, url: &String) -> Option<T> {
        if let Some(data) = read_from_cache::<T>(self._config.cache(), &url) {
            return Some(data);
        }
        match self.get_data::<T>(&url) {
            Some(data) => {
                write_to_cache::<T>(self._config.cache(), &url, &data);
                Some(data)
            }
            None => None,
        }
    }

    pub fn get_current_version(&self) -> Option<String> {
        let versions = self.get_data::<Vec<String>>(
            &"https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json".to_string(),
        );
        match versions {
            Some(vers) => Some(vers[0].as_str().to_string()),
            None => None,
        }
    }

    pub fn get_champ_data(&self, version: &String) -> Option<Box<HashMap<String, ChampionDatum>>> {
        let champ_data = self.get_cached_data::<Champions>(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json",
            version
        ));
        match champ_data {
            Some(data) => Some(Box::new(data.data)),
            None => None,
        }
    }

    pub fn get_items(&self, version: &String) -> Option<Box<HashMap<String, ItemDatum>>> {
        let champ_data = self.get_cached_data::<Items>(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/item.json",
            version
        ));
        match champ_data {
            Some(data) => Some(Box::new(data.data)),
            None => None,
        }
    }

    pub fn get_runes(&self, version: &String) -> Option<Box<HashMap<i64, RuneExtended>>> {
        let rune_data = self.get_cached_data::<RunePaths>(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/runesReforged.json",
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
                                parent_id: class.id.clone(),
                            };
                            processed_data.insert(rune.id, extended_rune);
                        }
                    }
                }
                Some(Box::new(processed_data))
            }
            None => None,
        }
    }

    pub fn get_summoner_spells(&self, version: &String) -> Option<Box<HashMap<i64, String>>> {
        let summoner_data = self.get_cached_data::<SummonerSpells>(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/summoner.json",
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
                Some(Box::new(reduced_data))
            }
            None => None,
        }
    }

    pub fn get_ugg_api_versions(&self, version: &String) -> Option<Box<UggAPIVersions>> {
        let ugg_api_version_endpoint =
            "https://static.u.gg/assets/lol/riot_patch_update/prod/ugg/ugg-api-versions.json"
                .to_string();
        let mut ugg_api_data = self.get_cached_data::<UggAPIVersions>(&ugg_api_version_endpoint);
        match ugg_api_data {
            Some(ugg_api) => {
                if !ugg_api.contains_key(version) {
                    clear_cache(self._config.cache(), &ugg_api_version_endpoint);
                    ugg_api_data =
                        self.get_cached_data::<UggAPIVersions>(&ugg_api_version_endpoint);
                    match ugg_api_data {
                        Some(ugg_api_retry) => Some(Box::new(ugg_api_retry)),
                        None => None,
                    }
                } else {
                    Some(Box::new(ugg_api))
                }
            }
            None => None,
        }
    }

    pub fn get_stats(
        &mut self,
        patch: &str,
        champ: &ChampionDatum,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
        api_versions: &HashMap<String, HashMap<String, String>>,
    ) -> Option<Box<(mappings::Role, OverviewData)>> {
        let mut api_version = "1.4.0";
        if api_versions.contains_key(patch) && api_versions[patch].contains_key("overview") {
            api_version = api_versions[patch]["overview"].as_str();
        }
        let data_path = &format!(
            "{}/{}/{}/{}",
            patch,
            mode.to_string(),
            champ.key.as_str(),
            api_version
        );
        let stats_data;
        match self._overview_lru_cache.get(&sha256(&data_path)) {
            Some(data) => stats_data = Some(data.clone()),
            None => {
                stats_data = self.get_data::<ChampOverview>(&format!(
                    "https://stats2.u.gg/lol/1.1/overview/{}.json",
                    data_path
                ));
            }
        }

        match stats_data {
            Some(champ_stats) => {
                self._overview_lru_cache
                    .put(sha256(data_path), champ_stats.clone());
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
                Some(Box::new((
                    role_query,
                    champ_stats[&region_query][&rank_query][&role_query]
                        .data
                        .clone(),
                )))
            }
            None => None,
        }
    }

    pub fn get_matchups(
        &mut self,
        patch: &str,
        champ: &ChampionDatum,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
        api_versions: &HashMap<String, HashMap<String, String>>,
    ) -> Option<Box<MatchupData>> {
        let mut api_version = "1.4.0";
        if api_versions.contains_key(patch) && api_versions[patch].contains_key("matchups") {
            api_version = api_versions[patch]["matchups"].as_str();
        }
        let data_path = &format!(
            "{}/{}/{}/{}",
            patch,
            mode.to_string(),
            champ.key.as_str(),
            api_version
        );
        let matchup_data;
        match self._matchup_lru_cache.get(&sha256(&data_path)) {
            Some(data) => matchup_data = Some(data.clone()),
            None => {
                matchup_data = self.get_data::<Matchups>(&format!(
                    "https://stats2.u.gg/lol/1.1/matchups/{}.json",
                    data_path
                ));
            }
        }

        match matchup_data {
            Some(champ_matchups) => {
                self._matchup_lru_cache
                    .put(sha256(data_path), champ_matchups.clone());
                let region_query = if champ_matchups.contains_key(&region) {
                    region
                } else {
                    mappings::Region::World
                };

                let rank_query =
                    if champ_matchups[&region_query].contains_key(&mappings::Rank::PlatinumPlus) {
                        mappings::Rank::PlatinumPlus
                    } else {
                        mappings::Rank::Overall
                    };

                Some(Box::new(
                    champ_matchups[&region_query][&rank_query][&role]
                        .data
                        .clone(),
                ))
            }
            None => None,
        }
    }
}
