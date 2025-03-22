use std::collections::HashMap;

use ddragon::models::champions::ChampionShort;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, Widget},
};
use ugg_types::arena_overview::{ArenaOverviewData, ChampionSynergy};

fn make_synergy_block<'a>(title: &str) -> Block<'a> {
    Block::default()
        .white()
        .title(format!(" {title} "))
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
}

pub fn make_placeholder() -> impl Widget {
    make_synergy_block("Champ Synergies")
}

fn make_list_from_champ_synergies<'a>(
    name: &str,
    synergies: &[ChampionSynergy],
    champs: &HashMap<String, ChampionShort>,
) -> List<'a> {
    List::new(
        synergies
            .iter()
            .filter_map(|i| {
                champs
                    .get(&i.id.to_string())
                    .map(|it| ListItem::new(it.name.clone()))
            })
            .take(10)
            .collect::<Vec<_>>(),
    )
    .block(make_synergy_block(name))
}

pub fn make(overview: &ArenaOverviewData, champs: &HashMap<String, ChampionShort>) -> impl Widget {
    make_list_from_champ_synergies("Champ Synergies", &overview.champion_synergies, champs)
}
