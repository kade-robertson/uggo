use std::{collections::HashMap, time::Duration};

use ddragon::models::champions::ChampionShort;
use ratatui::widgets::ListItem;
use tui_input::Input;
use ugg_types::{
    client_runepage::NewRunePage,
    mappings::{Mode, Region, Role},
    matchups::MatchupData,
    overview::OverviewData,
};
use uggo_config::Config;
use uggo_lol_client::LOLClientAPI;
use uggo_ugg_api::{UggApi, UggApiBuilder};

use crate::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initial,
    TextInput,
    ChampScroll,
    ChampSelected,
    ModeSelect,
    VersionSelect,
}

pub struct AppContext<'a> {
    pub api: UggApi,
    pub client_api: Option<LOLClientAPI>,
    pub state: State,
    pub champ_scroll_pos: Option<usize>,
    pub champ_data: Vec<(usize, ChampionShort)>,
    pub champ_by_key: HashMap<String, ChampionShort>,
    pub list_indices: Vec<usize>,
    pub champ_list: Vec<ListItem<'a>>,
    pub selected_champ: Option<ChampionShort>,
    pub selected_champ_overview: Option<OverviewData>,
    pub selected_champ_matchups: Option<MatchupData>,
    pub max_item_length: usize,
    pub items: Vec<String>,
    pub input: Input,
    pub mode: Mode,
    pub mode_scroll_pos: Option<usize>,
    pub version: String,
    pub version_scroll_pos: Option<usize>,
    pub last_render_duration: Option<Duration>,
}

impl AppContext<'_> {
    fn create(api: UggApi) -> Self {
        let version = api.current_version.clone();
        let version_index = api
            .allowed_versions
            .iter()
            .position(|v| v.ddragon == version);

        let mut ordered_champ_data = api
            .champ_data
            .values()
            .enumerate()
            .map(|(i, c)| (i, c.clone()))
            .collect::<Vec<_>>();
        ordered_champ_data.sort_by(|(_, a), (_, b)| a.name.cmp(&b.name));

        let mut ordered_item_names = api
            .items
            .values()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>();
        ordered_item_names.sort_by_key(std::string::String::len);
        ordered_item_names.reverse();

        let max_item_length = ordered_item_names
            .first()
            .map(std::string::String::len)
            .unwrap_or_default();

        let champ_by_key = api
            .champ_data
            .values()
            .map(|c| (c.key.clone(), c.clone()))
            .collect::<HashMap<_, _>>();

        let mut app_context = Self {
            api,
            client_api: LOLClientAPI::new().ok(),
            state: State::Initial,
            champ_scroll_pos: None,
            champ_data: ordered_champ_data,
            champ_by_key,
            list_indices: Vec::new(),
            champ_list: Vec::new(),
            input: Input::default(),
            selected_champ: None,
            selected_champ_overview: None,
            selected_champ_matchups: None,
            max_item_length,
            items: ordered_item_names,
            mode: Mode::Normal,
            mode_scroll_pos: None,
            version,
            version_scroll_pos: version_index,
            last_render_duration: None,
        };
        app_context.update_champ_list();

        app_context
    }

    pub fn new_with_version(version: &str) -> anyhow::Result<Self> {
        let config = Config::new()?;
        let api = UggApiBuilder::new()
            .version(version)
            .cache_dir(config.cache())
            .build()?;
        Ok(Self::create(api))
    }

    pub fn new() -> anyhow::Result<Self> {
        let config = Config::new()?;
        let api = UggApiBuilder::new().cache_dir(config.cache()).build()?;
        Ok(Self::create(api))
    }

    pub fn replace(&mut self, version: &str) -> anyhow::Result<()> {
        *self = Self::new_with_version(version)?;
        Ok(())
    }

    pub fn update_champ_list(&mut self) {
        (self.list_indices, self.champ_list) = self
            .champ_data
            .iter()
            .filter(|(_, c)| {
                c.name
                    .to_lowercase()
                    .contains(&self.input.value().to_lowercase())
            })
            .map(|(i, c)| (i, ListItem::new(c.name.clone())))
            .unzip();
    }

    pub fn select_champion(&mut self, champ: &ChampionShort) {
        self.champ_scroll_pos = None;
        self.selected_champ = Some(champ.clone());
        self.selected_champ_overview = self
            .api
            .get_stats(champ, Role::Automatic, Region::World, self.mode)
            .map(|v| *v)
            .ok();
        if self.mode == Mode::ARAM {
            self.selected_champ_matchups = None;
        } else {
            self.selected_champ_matchups = self
                .api
                .get_matchups(champ, Role::Automatic, Region::World, self.mode)
                .map(|v| *v)
                .ok();
        }

        if let Some(ref overview) = self.selected_champ_overview {
            if let Some(ref api) = self.client_api {
                if let Some(data) = api.get_current_rune_page() {
                    let (primary_style_id, sub_style_id, selected_perk_ids) =
                        util::generate_perk_array(
                            &util::group_runes(&overview.runes.rune_ids, &self.api.runes),
                            &overview.shards.shard_ids,
                        );
                    api.update_rune_page(
                        data.id,
                        &NewRunePage {
                            name: format!("uggo: {}, {}", &champ.name, self.mode),
                            primary_style_id,
                            sub_style_id,
                            selected_perk_ids,
                        },
                    );
                }
            }
        }

        self.state = State::ChampSelected;
    }

    #[cfg(debug_assertions)]
    pub fn set_render_duration(&mut self, duration: Duration) {
        self.last_render_duration = Some(duration);
    }
}