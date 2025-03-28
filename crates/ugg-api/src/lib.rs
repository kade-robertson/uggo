use crate::util::sha256;
use ddragon::models::Augment;
use ddragon::models::champions::ChampionShort;
use ddragon::models::items::Item;
use ddragon::models::runes::RuneElement;
use ddragon::{Client, ClientBuilder};
use levenshtein::levenshtein;
use lru::LruCache;
use serde::de::DeserializeOwned;
use std::cell::RefCell;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use thiserror::Error;
use ugg_types::mappings::{self, Rank};
use ugg_types::matchups::{MatchupData, Matchups};
use ugg_types::overview::{ChampOverview, Overview};
use ugg_types::rune::RuneExtended;
use ureq::Agent;

mod util;

type UggAPIVersions = HashMap<String, HashMap<String, String>>;

#[derive(Error, Debug)]
pub enum UggError {
    #[error("DDragon error")]
    DDragonError(#[from] ddragon::ClientError),
    #[error("HTTP request failed")]
    RequestError(#[from] Box<ureq::Error>),
    #[error("JSON parsing failed")]
    ParseError(#[from] simd_json::Error),
    #[error("Missing region or rank entry")]
    MissingRegionOrRank,
    #[error("Missing role entry")]
    MissingRole,
    #[error("Unknown error occurred")]
    Unknown,
}

pub struct DataApi {
    agent: Agent,
    ddragon: Client,
    overview_cache: RefCell<LruCache<String, ChampOverview>>,
    matchup_cache: RefCell<LruCache<String, Matchups>>,
}

#[derive(Debug, Clone)]
pub struct SupportedVersion {
    pub ddragon: String,
    pub ugg: String,
}

pub struct UggApi {
    api: DataApi,
    ugg_api_versions: UggAPIVersions,

    pub current_version: String,
    pub allowed_versions: Vec<SupportedVersion>,
    pub patch_version: String,
    pub champ_data: HashMap<String, ChampionShort>,
    pub items: HashMap<String, Item>,
    pub runes: HashMap<i64, RuneExtended<RuneElement>>,
    pub summoner_spells: HashMap<i64, String>,
    pub arena_augments: HashMap<i64, Augment>,
}

impl DataApi {
    pub fn new(version: Option<String>, cache_dir: Option<PathBuf>) -> Result<Self, UggError> {
        let mut client_builder = ClientBuilder::new();
        let safe_dir = cache_dir.ok_or(UggError::Unknown)?;
        if let Some(v) = version {
            client_builder = client_builder.version(v.as_str());
        }
        if let Some(dir) = safe_dir.clone().to_str() {
            client_builder = client_builder.cache(dir);
        }

        let cache_size = NonZeroUsize::new(50).unwrap_or(NonZeroUsize::MIN);
        Ok(Self {
            agent: Agent::new_with_defaults(),
            ddragon: client_builder.build()?,
            overview_cache: RefCell::new(LruCache::new(cache_size)),
            matchup_cache: RefCell::new(LruCache::new(cache_size)),
        })
    }

    fn get_data<T: DeserializeOwned>(&self, url: &str) -> Result<T, UggError> {
        simd_json::serde::from_reader::<ureq::BodyReader<'_>, T>(
            self.agent
                .get(url)
                .call()
                .map_err(Box::new)?
                .into_body()
                .as_reader(),
        )
        .map_err(UggError::ParseError)
    }

    pub fn get_current_version(&mut self) -> String {
        self.ddragon.version.clone()
    }

    pub fn get_supported_versions(&self) -> Result<Vec<String>, UggError> {
        self.get_data("https://ddragon.leagueoflegends.com/api/versions.json")
    }

    pub fn get_champ_data(&self) -> Result<HashMap<String, ChampionShort>, UggError> {
        Ok(self.ddragon.champions()?.data)
    }

    pub fn get_items(&self) -> Result<HashMap<String, Item>, UggError> {
        Ok(self.ddragon.items()?.data)
    }

    pub fn get_runes(&self) -> Result<HashMap<i64, RuneExtended<RuneElement>>, UggError> {
        let rune_data = self.ddragon.runes()?;

        let mut processed_data = HashMap::new();
        for class in rune_data {
            for (slot_index, slot) in class.slots.iter().enumerate() {
                for (index, rune) in slot.runes.iter().enumerate() {
                    let extended_rune = RuneExtended {
                        rune: (*rune).clone(),
                        slot: slot_index as u64,
                        index: index as u64,
                        siblings: slot.runes.len() as u64,
                        parent: class.name.clone(),
                        parent_id: class.id,
                    };
                    processed_data.insert(rune.id, extended_rune);
                }
            }
        }
        Ok(processed_data)
    }

    pub fn get_summoner_spells(&self) -> Result<HashMap<i64, String>, UggError> {
        let summoner_data = self.ddragon.summoner_spells()?;

        let mut reduced_data: HashMap<i64, String> = HashMap::new();
        for (_spell, spell_info) in summoner_data.data {
            reduced_data.insert(
                spell_info.key.parse::<i64>().ok().unwrap_or(0),
                spell_info.name,
            );
        }
        Ok(reduced_data)
    }

    pub fn get_arena_augments(&self) -> Result<HashMap<i64, Augment>, UggError> {
        let augment_data = self.ddragon.arena_augments()?;
        let mut reduced_data: HashMap<i64, Augment> = HashMap::new();
        for augment in augment_data {
            reduced_data.insert(augment.id, augment);
        }
        Ok(reduced_data)
    }

    pub fn get_ugg_api_versions(&self) -> Result<UggAPIVersions, UggError> {
        self.get_data::<UggAPIVersions>("https://static.bigbrain.gg/assets/lol/riot_patch_update/prod/ugg/ugg-api-versions.json")
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_stats(
        &self,
        patch: &str,
        champ: &ChampionShort,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
        build: mappings::Build,
        api_versions: &HashMap<String, HashMap<String, String>>,
    ) -> Result<(Overview, mappings::Role), UggError> {
        let api_version =
            if api_versions.contains_key(patch) && api_versions[patch].contains_key("overview") {
                api_versions[patch]["overview"].as_str()
            } else {
                "1.5.0"
            };
        let data_path = &format!(
            "{}/{}/{}/{}/{}",
            build.to_api_string(),
            patch,
            mode.to_api_string(),
            champ.key.as_str(),
            api_version
        );
        let cache_path = format!("{data_path}-{region}-{role}");

        let stats_data = if let Some(data) = self
            .overview_cache
            .try_borrow_mut()
            .ok()
            .and_then(|mut c| c.get(&sha256(&cache_path)).cloned())
        {
            Ok(data)
        } else {
            self.get_data::<ChampOverview>(&format!("https://stats2.u.gg/lol/1.5/{data_path}.json"))
        }?;

        if let Ok(mut c) = self.overview_cache.try_borrow_mut() {
            c.put(sha256(&cache_path), stats_data.clone());
        }

        let data_by_role = Rank::preferred_order()
            .iter()
            .find_map(|rank| {
                stats_data
                    .get(&region)
                    .and_then(|region_data| region_data.get(rank))
            })
            .ok_or(UggError::MissingRegionOrRank)?;

        data_by_role
            .get_key_value(&role)
            .or_else(|| {
                data_by_role
                    .iter()
                    .max_by_key(|(_, data)| data.data.matches())
                    .map(|(role, _)| role)
                    .and_then(|r| data_by_role.get_key_value(r))
            })
            .map(|(role, data)| (data.data.clone(), *role))
            .ok_or(UggError::MissingRole)
    }

    pub fn get_matchups(
        &self,
        patch: &str,
        champ: &ChampionShort,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
        api_versions: &HashMap<String, HashMap<String, String>>,
    ) -> Result<(MatchupData, mappings::Role), UggError> {
        let api_version =
            if api_versions.contains_key(patch) && api_versions[patch].contains_key("matchups") {
                api_versions[patch]["matchups"].as_str()
            } else {
                "1.5.0"
            };
        let data_path = &format!(
            "{}/{}/{}/{}",
            patch,
            mode.to_api_string(),
            champ.key.as_str(),
            api_version
        );
        let cache_path = format!("{data_path}-{region}-{role}");

        let matchup_data = if let Some(data) = self
            .matchup_cache
            .try_borrow_mut()
            .ok()
            .and_then(|mut c| c.get(&sha256(&cache_path)).cloned())
        {
            Ok(data)
        } else {
            self.get_data::<Matchups>(&format!(
                "https://stats2.u.gg/lol/1.5/matchups/{data_path}.json",
            ))
        }?;

        let data_by_role = Rank::preferred_order()
            .iter()
            .find_map(|rank| {
                matchup_data
                    .get(&region)
                    .and_then(|region_data| region_data.get(rank))
            })
            .ok_or(UggError::MissingRegionOrRank)?;

        data_by_role
            .get_key_value(&role)
            .or_else(|| {
                data_by_role
                    .iter()
                    .max_by_key(|(_, data)| data.data.total_matches)
                    .map(|(role, _)| role)
                    .and_then(|r| data_by_role.get_key_value(r))
            })
            .map(|(role, data)| (data.data.clone(), *role))
            .ok_or(UggError::MissingRole)
    }
}

impl UggApi {
    pub fn new(version: Option<String>, cache_dir: Option<PathBuf>) -> Result<Self, UggError> {
        let mut inner_api = DataApi::new(version, cache_dir.clone())?;

        let mut current_version = inner_api.get_current_version();
        let allowed_versions = inner_api.get_supported_versions()?;
        let ugg_api_versions = inner_api.get_ugg_api_versions()?;
        let versions_ugg_supports = allowed_versions
            .into_iter()
            .map(|v| SupportedVersion {
                ddragon: v.clone(),
                ugg: (v.split('.').take(2).collect::<Vec<&str>>()).join("_"),
            })
            .filter(|v| ugg_api_versions.contains_key(&v.ugg))
            .collect::<Vec<_>>();

        if let Some(default_if_fails) = versions_ugg_supports.first() {
            if !versions_ugg_supports
                .iter()
                .any(|v| v.ddragon == current_version)
            {
                inner_api = DataApi::new(Some(default_if_fails.ddragon.clone()), cache_dir)?;
                current_version = inner_api.get_current_version();
            }
        } else {
            return Err(UggError::Unknown);
        }

        let champ_data = inner_api.get_champ_data()?;
        let items = inner_api.get_items()?;
        let runes = inner_api.get_runes()?;
        let summoner_spells = inner_api.get_summoner_spells()?;
        let arena_augments = inner_api.get_arena_augments()?;

        let mut patch_version_split = current_version.split('.').collect::<Vec<&str>>();
        patch_version_split.remove(patch_version_split.len() - 1);
        let patch_version = patch_version_split.join("_");

        Ok(Self {
            api: inner_api,
            allowed_versions: versions_ugg_supports,
            ugg_api_versions,
            current_version,
            patch_version,
            champ_data,
            items,
            runes,
            summoner_spells,
            arena_augments,
        })
    }

    pub fn find_champ(&self, name: &str) -> &ChampionShort {
        if self.champ_data.contains_key(name) {
            &self.champ_data[name]
        } else {
            let mut lowest_distance = usize::MAX;
            let mut closest_champ: &ChampionShort = &self.champ_data["Annie"];

            let mut substring_lowest_dist = usize::MAX;
            let mut substring_closest_champ: Option<&ChampionShort> = None;

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
        champ: &ChampionShort,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
        build: mappings::Build,
    ) -> Result<(Overview, mappings::Role), UggError> {
        self.api.get_stats(
            &self.patch_version,
            champ,
            role,
            region,
            mode,
            build,
            &self.ugg_api_versions,
        )
    }

    pub fn get_matchups(
        &self,
        champ: &ChampionShort,
        role: mappings::Role,
        region: mappings::Region,
        mode: mappings::Mode,
    ) -> Result<(MatchupData, mappings::Role), UggError> {
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

pub struct UggApiBuilder {
    version: Option<String>,
    cache_dir: Option<PathBuf>,
}

impl UggApiBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            version: None,
            cache_dir: None,
        }
    }

    #[must_use]
    pub fn version(mut self, version: &str) -> Self {
        self.version = Some(version.to_owned());
        self
    }

    #[must_use]
    pub fn cache_dir(mut self, cache_dir: &Path) -> Self {
        self.cache_dir = Some(cache_dir.to_path_buf());
        self
    }

    pub fn build(self) -> Result<UggApi, UggError> {
        UggApi::new(self.version, self.cache_dir)
    }
}

impl Default for UggApiBuilder {
    fn default() -> Self {
        Self::new()
    }
}
