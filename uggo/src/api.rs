use crate::config::Config;
use crate::util::{clear_cache, read_from_cache, sha256, write_to_cache};
use anyhow::{anyhow, Result};
use levenshtein::levenshtein;
use lru::LruCache;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use ugg_types::champion::{ChampionDatum, Champions};
use ugg_types::item::{ItemDatum, Items};
use ugg_types::mappings;
use ugg_types::matchups::MatchupData;
use ugg_types::overview::OverviewData;
use ugg_types::rune::{RuneExtended, RunePaths};
use ugg_types::summonerspell::SummonerSpells;
use ureq::Agent;

type UggAPIVersions = HashMap<String, HashMap<String, String>>;

pub struct DataApi {
    agent: Agent,
    config: Config,
    overview_cache: RefCell<LruCache<String, OverviewData>>,
    matchup_cache: RefCell<LruCache<String, MatchupData>>,
}

pub struct UggApi {
    api: DataApi,
    ugg_api_versions: UggAPIVersions,

    pub current_version: String,
    pub patch_version: String,
    pub champ_data: HashMap<String, ChampionDatum>,
    pub items: HashMap<String, ItemDatum>,
    pub runes: HashMap<i64, RuneExtended>,
    pub summoner_spells: HashMap<i64, String>,
}

impl DataApi {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
            config: Config::new(),
            overview_cache: RefCell::new(LruCache::new(NonZeroUsize::new(25).unwrap())),
            matchup_cache: RefCell::new(LruCache::new(NonZeroUsize::new(25).unwrap())),
        }
    }

    fn get_data<T: DeserializeOwned>(&self, url: &String) -> Result<T> {
        let response = self.agent.get(url).call()?;
        response
            .into_json::<T>()
            .map_or_else(|_| Err(anyhow!("Could not fetch {}", url)), |e| Ok(e))
    }

    fn get_cached_data<T: DeserializeOwned + Serialize>(&self, url: &String) -> Result<T> {
        if let Some(data) = read_from_cache::<T>(self.config.cache(), url) {
            return Ok(data);
        }
        match self.get_data::<T>(url) {
            Ok(data) => {
                write_to_cache::<T>(self.config.cache(), url, &data);
                Ok(data)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_current_version(&mut self) -> Result<String> {
        let versions = self.get_data::<Vec<String>>(
            &"https://static.bigbrain.gg/assets/lol/riot_patch_update/prod/versions.json"
                .to_string(),
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

    pub fn get_ugg_api_versions(&self, version: &String) -> Result<UggAPIVersions> {
        let ugg_api_version_endpoint =
            "https://static.bigbrain.gg/assets/lol/riot_patch_update/prod/ugg/ugg-api-versions.json"
                .to_string();
        let mut ugg_api_data = self.get_cached_data::<UggAPIVersions>(&ugg_api_version_endpoint);

        match ugg_api_data {
            Ok(ugg_api) => {
                if ugg_api.contains_key(version) {
                    Ok(ugg_api)
                } else {
                    clear_cache(self.config.cache(), &ugg_api_version_endpoint);
                    ugg_api_data =
                        self.get_cached_data::<UggAPIVersions>(&ugg_api_version_endpoint);
                    ugg_api_data
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_stats(
        &self,
        patch: &str,
        champ: &ChampionDatum,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
        api_versions: &HashMap<String, HashMap<String, String>>,
    ) -> Result<Box<OverviewData>> {
        let api_version =
            if api_versions.contains_key(patch) && api_versions[patch].contains_key("overview") {
                api_versions[patch]["overview"].as_str()
            } else {
                "1.4.0"
            };
        let data_path = &format!(
            "{}/{}/{}/{}",
            patch,
            mode.to_api_string(),
            champ.key.as_str(),
            api_version
        );

        let stats_data = match self
            .overview_cache
            .try_borrow_mut()?
            .get(&sha256(data_path))
        {
            Some(data) => Ok(data.clone()),
            None => self.get_data::<OverviewData>(&format!(
                "https://ugg-proxy.fly.dev/{}/overview.json?region={}&role={}",
                data_path, region as i32, role as i32
            )),
        }?;

        if let Ok(mut c) = self.overview_cache.try_borrow_mut() {
            c.put(sha256(data_path), stats_data.clone());
        }

        Ok(Box::new(stats_data))
    }

    pub fn get_matchups(
        &self,
        patch: &str,
        champ: &ChampionDatum,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
        api_versions: &HashMap<String, HashMap<String, String>>,
    ) -> Result<Box<MatchupData>> {
        let api_version =
            if api_versions.contains_key(patch) && api_versions[patch].contains_key("matchups") {
                api_versions[patch]["matchups"].as_str()
            } else {
                "1.4.0"
            };
        let data_path = &format!(
            "{}/{}/{}/{}",
            patch,
            mode.to_api_string(),
            champ.key.as_str(),
            api_version
        );

        let matchup_data = match self.matchup_cache.try_borrow_mut()?.get(&sha256(data_path)) {
            Some(data) => Ok(data.clone()),
            None => self.get_data::<MatchupData>(&format!(
                "https://ugg-proxy.fly.dev/{}/matchups.json?region={}&role={}",
                data_path, region as i32, role as i32
            )),
        }?;

        if let Ok(mut c) = self.matchup_cache.try_borrow_mut() {
            c.put(sha256(data_path), matchup_data.clone());
        }

        Ok(Box::new(matchup_data))
    }
}

impl UggApi {
    pub fn new() -> Result<Self> {
        let mut inner_api = DataApi::new();

        let current_version = inner_api.get_current_version()?;
        let champ_data = inner_api.get_champ_data(&current_version)?;
        let items = inner_api.get_items(&current_version)?;
        let runes = inner_api.get_runes(&current_version)?;
        let summoner_spells = inner_api.get_summoner_spells(&current_version)?;

        let mut patch_version_split = current_version.split('.').collect::<Vec<&str>>();
        patch_version_split.remove(patch_version_split.len() - 1);
        let patch_version = patch_version_split.join("_");

        let ugg_api_versions = inner_api.get_ugg_api_versions(&patch_version)?;

        Ok(Self {
            api: inner_api,
            ugg_api_versions,
            current_version,
            patch_version,
            champ_data,
            items,
            runes,
            summoner_spells,
        })
    }

    pub fn find_champ(&self, name: &str) -> &ChampionDatum {
        if self.champ_data.contains_key(name) {
            &self.champ_data[name]
        } else {
            let mut lowest_distance = usize::MAX;
            let mut closest_champ: &ChampionDatum = &self.champ_data["Annie"];

            let mut substring_lowest_dist = usize::MAX;
            let mut substring_closest_champ: Option<&ChampionDatum> = None;

            for value in self.champ_data.values() {
                let query_compare = name.to_ascii_lowercase();
                let champ_compare = value.name.to_ascii_lowercase();
                // Prefer matches where search query is an exact starting substring
                let distance = levenshtein(query_compare.as_str(), champ_compare.as_str());
                if champ_compare.starts_with(&query_compare) {
                    if distance <= substring_lowest_dist {
                        substring_lowest_dist = distance;
                        substring_closest_champ = Some(value);
                    }
                } else if distance <= lowest_distance {
                    lowest_distance = distance;
                    closest_champ = value;
                }
            }

            substring_closest_champ.unwrap_or(closest_champ)
        }
    }

    pub fn get_stats(
        &self,
        champ: &ChampionDatum,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
    ) -> Result<Box<OverviewData>> {
        self.api.get_stats(
            &self.patch_version,
            champ,
            role,
            region,
            mode,
            &self.ugg_api_versions,
        )
    }

    pub fn get_matchups(
        &self,
        champ: &ChampionDatum,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
    ) -> Result<Box<MatchupData>> {
        self.api.get_matchups(
            &self.patch_version,
            champ,
            role,
            region,
            mode,
            &self.ugg_api_versions,
        )
    }
}