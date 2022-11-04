use crate::config::Config;
use crate::mappings;
use crate::types::champion::{ChampionDatum, Champions};
use crate::types::item::{ItemDatum, Items};
use crate::types::matchups::{MatchupData, Matchups};
use crate::types::overview::{ChampOverview, OverviewData};
use crate::types::rune::{RuneExtended, RunePaths};
use crate::types::summonerspell::SummonerSpells;
use crate::util::{clear_cache, read_from_cache, sha256, write_to_cache};
use anyhow::{anyhow, Result};
use lru::LruCache;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

type UggAPIVersions = HashMap<String, HashMap<String, String>>;

pub struct DataApi {
    _client: Client,
    _config: Config,
    _overview_lru_cache: LruCache<String, ChampOverview>,
    _matchup_lru_cache: LruCache<String, Matchups>,
}

impl DataApi {
    pub fn new() -> DataApi {
        DataApi {
            _client: Client::new(),
            _config: Config::new(),
            _overview_lru_cache: LruCache::new(25),
            _matchup_lru_cache: LruCache::new(25),
        }
    }

    fn get_data<T: DeserializeOwned>(&self, url: &String) -> Result<T> {
        let response = self._client.get(url).send()?;
        match response.json::<T>() {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("Could not fetch {}", url)),
        }
    }

    fn get_cached_data<T: DeserializeOwned + Serialize>(&self, url: &String) -> Result<T> {
        if let Some(data) = read_from_cache::<T>(self._config.cache(), url) {
            return Ok(data);
        }
        match self.get_data::<T>(url) {
            Ok(data) => {
                write_to_cache::<T>(self._config.cache(), url, &data);
                Ok(data)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_current_version(&self) -> Result<String> {
        let versions = self.get_data::<Vec<String>>(
            &"https://static.u.gg/assets/lol/riot_patch_update/prod/versions.json".to_string(),
        );

        versions.map(|vers| vers[0].as_str().to_string())
    }

    pub fn get_champ_data(&self, version: &String) -> Result<HashMap<String, ChampionDatum>> {
        let champ_data = self.get_cached_data::<Champions>(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json",
            version
        ));

        champ_data.map(|d| d.data)
    }

    pub fn get_items(&self, version: &String) -> Result<HashMap<String, ItemDatum>> {
        let champ_data = self.get_cached_data::<Items>(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/item.json",
            version
        ));

        champ_data.map(|d| d.data)
    }

    pub fn get_runes(&self, version: &String) -> Result<HashMap<i64, RuneExtended>> {
        let rune_data = self.get_cached_data::<RunePaths>(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/runesReforged.json",
            version
        ));
        match rune_data {
            Ok(data) => {
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
                                parent_id: class.id,
                            };
                            processed_data.insert(rune.id, extended_rune);
                        }
                    }
                }
                Ok(processed_data)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_summoner_spells(&self, version: &String) -> Result<HashMap<i64, String>> {
        let summoner_data = self.get_cached_data::<SummonerSpells>(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/summoner.json",
            version
        ));
        match summoner_data {
            Ok(spells) => {
                let mut reduced_data: HashMap<i64, String> = HashMap::new();
                for (_spell, spell_info) in spells.data {
                    reduced_data.insert(
                        spell_info.key.parse::<i64>().ok().unwrap_or(0),
                        spell_info.name,
                    );
                }
                Ok(reduced_data)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_ugg_api_versions(&self, version: &String) -> Result<Box<UggAPIVersions>> {
        let ugg_api_version_endpoint =
            "https://static.u.gg/assets/lol/riot_patch_update/prod/ugg/ugg-api-versions.json"
                .to_string();
        let mut ugg_api_data = self.get_cached_data::<UggAPIVersions>(&ugg_api_version_endpoint);

        match ugg_api_data {
            Ok(ugg_api) => {
                if !ugg_api.contains_key(version) {
                    clear_cache(self._config.cache(), &ugg_api_version_endpoint);
                    ugg_api_data =
                        self.get_cached_data::<UggAPIVersions>(&ugg_api_version_endpoint);
                    ugg_api_data.map(Box::new)
                } else {
                    Ok(Box::new(ugg_api))
                }
            }
            Err(e) => Err(e),
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
    ) -> Result<Box<(mappings::Role, OverviewData)>> {
        let mut api_version = "1.4.0";
        if api_versions.contains_key(patch) && api_versions[patch].contains_key("overview") {
            api_version = api_versions[patch]["overview"].as_str();
        }
        let data_path = &format!(
            "{}/{}/{}/{}",
            patch,
            mode.to_api_string(),
            champ.key.as_str(),
            api_version
        );
        let stats_data = match self._overview_lru_cache.get(&sha256(data_path)) {
            Some(data) => Ok(data.clone()),
            None => self.get_data::<ChampOverview>(&format!(
                "https://stats2.u.gg/lol/1.5/overview/{}.json",
                data_path
            )),
        }?;

        self._overview_lru_cache
            .put(sha256(data_path), stats_data.clone());
        let region_query = if stats_data.contains_key(&region) {
            region
        } else {
            mappings::Region::World
        };

        let rank_query = if stats_data[&region_query].contains_key(&mappings::Rank::PlatinumPlus) {
            mappings::Rank::PlatinumPlus
        } else {
            mappings::Rank::Overall
        };

        let mut role_query = role;
        if !stats_data[&region_query][&rank_query].contains_key(&role_query) {
            if role_query == mappings::Role::Automatic {
                // Go through each role and pick the one with most matches played
                let mut most_games = 0;
                let mut used_role = role;
                for (role_key, role_stats) in &stats_data[&region_query][&rank_query] {
                    if role_stats.data.matches > most_games {
                        most_games = role_stats.data.matches;
                        used_role = *role_key;
                    }
                }
                role_query = used_role;
            } else {
                // This should only happen in ARAM
                role_query = mappings::Role::None;
            }
        }

        let stats = stats_data
            .get(&region_query)
            .ok_or_else(|| anyhow!("Region missing"))?
            .get(&rank_query)
            .ok_or_else(|| anyhow!("Rank missing"))?
            .get(&role_query)
            .ok_or_else(|| anyhow!("Role missing"))?;

        Ok(Box::new((role_query, stats.data.clone())))
    }

    pub fn get_matchups(
        &mut self,
        patch: &str,
        champ: &ChampionDatum,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
        api_versions: &HashMap<String, HashMap<String, String>>,
    ) -> Result<Box<MatchupData>> {
        let mut api_version = "1.4.0";
        if api_versions.contains_key(patch) && api_versions[patch].contains_key("matchups") {
            api_version = api_versions[patch]["matchups"].as_str();
        }
        let data_path = &format!(
            "{}/{}/{}/{}",
            patch,
            mode.to_api_string(),
            champ.key.as_str(),
            api_version
        );
        let matchup_data = match self._matchup_lru_cache.get(&sha256(data_path)) {
            Some(data) => Ok(data.clone()),
            None => self.get_data::<Matchups>(&format!(
                "https://stats2.u.gg/lol/1.5/matchups/{}.json",
                data_path
            )),
        }?;

        self._matchup_lru_cache
            .put(sha256(data_path), matchup_data.clone());
        let region_query = if matchup_data.contains_key(&region) {
            region
        } else {
            mappings::Region::World
        };

        let rank_query = if matchup_data[&region_query].contains_key(&mappings::Rank::PlatinumPlus)
        {
            mappings::Rank::PlatinumPlus
        } else {
            mappings::Rank::Overall
        };

        let matchups = matchup_data
            .get(&region_query)
            .ok_or_else(|| anyhow!("Region missing"))?
            .get(&rank_query)
            .ok_or_else(|| anyhow!("Rank missing"))?
            .get(&role)
            .ok_or_else(|| anyhow!("Role missing"))?;

        Ok(Box::new(matchups.data.clone()))
    }
}
