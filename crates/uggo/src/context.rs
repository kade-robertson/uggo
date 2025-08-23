use std::collections::HashMap;

#[cfg(debug_assertions)]
use std::time::Duration;

use ddragon::models::champions::ChampionShort;
use ratatui::widgets::ListItem;
use tui_input::Input;
use tui_logger::TuiWidgetState;
use ugg_types::{
    client_runepage::NewRunePage,
    mappings::{Build, Mode, Region, Role},
    matchups::MatchupData,
    overview::Overview,
};
use uggo_config::Config;
use uggo_lol_client::LOLClientAPI;
use uggo_ugg_api::{UggApi, UggApiBuilder};

use crate::transpose::Transposable;
use crate::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initial,
    TextInput,
    ChampScroll,
    ChampSelected,
    ModeSelect,
    VersionSelect,
    RegionSelect,
    RoleSelect,
    BuildSelect,
    HelpMenu,
    Logger,
}

pub struct AppContext<'a> {
    pub api: UggApi,
    pub client_api: Option<LOLClientAPI>,
    pub state: State,
    pub show_left_pane: bool,
    pub champ_scroll_pos: Option<usize>,
    pub champ_data: Vec<(usize, ChampionShort)>,
    pub champ_by_key: HashMap<String, ChampionShort>,
    pub list_indices: Vec<usize>,
    pub champ_list: Vec<ListItem<'a>>,
    pub selected_champ: Option<ChampionShort>,
    pub selected_champ_overview: Option<Overview>,
    pub selected_champ_role: Option<Role>,
    pub selected_champ_matchups: Option<MatchupData>,
    pub input: Input,
    pub mode: Mode,
    pub mode_scroll_pos: Option<usize>,
    pub version: String,
    pub version_scroll_pos: Option<usize>,
    pub region: Region,
    pub region_scroll_pos: Option<usize>,
    pub role: Role,
    pub role_scroll_pos: Option<usize>,
    pub build: Build,
    pub build_scroll_pos: Option<usize>,
    pub logger_state: TuiWidgetState,
    #[cfg(debug_assertions)]
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

        let champ_by_key = api
            .champ_data
            .values()
            .map(|c| (c.key.clone(), c.clone()))
            .collect::<HashMap<_, _>>();

        let mut app_context = Self {
            api,
            client_api: LOLClientAPI::new().ok(),
            state: State::Initial,
            show_left_pane: true,
            champ_scroll_pos: None,
            champ_data: ordered_champ_data,
            champ_by_key,
            list_indices: Vec::new(),
            champ_list: Vec::new(),
            input: Input::default(),
            selected_champ: None,
            selected_champ_overview: None,
            selected_champ_role: None,
            selected_champ_matchups: None,
            mode: Mode::Normal,
            mode_scroll_pos: None,
            version,
            version_scroll_pos: version_index,
            region: Region::World,
            region_scroll_pos: Region::all().iter().position(|r| r == &Region::World),
            role: Role::Automatic,
            role_scroll_pos: Role::all().iter().position(|r| r == &Role::Automatic),
            build: Build::Recommended,
            build_scroll_pos: Build::all().iter().position(|r| r == &Build::Recommended),
            logger_state: TuiWidgetState::default(),
            #[cfg(debug_assertions)]
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

    pub fn return_to_initial(&mut self, reset_champ_scroll: bool) {
        self.state = State::Initial;
        if reset_champ_scroll {
            self.champ_scroll_pos = None;
        }
    }

    pub fn select_champion(&mut self, champ: &ChampionShort) {
        self.champ_scroll_pos = None;
        self.selected_champ = Some(champ.clone());
        (self.selected_champ_overview, self.selected_champ_role) = self
            .api
            .get_stats(champ, self.role, self.region, self.mode, self.build)
            .ok()
            .transpose();
        if self.mode == Mode::ARAM || self.mode == Mode::Arena {
            self.selected_champ_matchups = None;
        } else {
            self.selected_champ_matchups = self
                .api
                .get_matchups(champ, self.role, self.region, self.mode)
                .map(|v| v.0)
                .ok();
        }

        if let Some(Overview::Default(ref overview)) = self.selected_champ_overview
            && let Some(ref api) = self.client_api
            && let Some(data) = api.get_current_rune_page()
        {
            let (primary_style_id, sub_style_id, selected_perk_ids) = util::generate_perk_array(
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

        self.state = State::ChampSelected;
    }

    #[cfg(debug_assertions)]
    pub fn set_render_duration(&mut self, duration: Duration) {
        self.last_render_duration = Some(duration);
    }
}
