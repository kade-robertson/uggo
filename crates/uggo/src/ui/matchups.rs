use std::{borrow::Cow, collections::HashMap};

use ddragon::models::champions::ChampionShort;
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Paragraph, Widget},
};
use ugg_types::matchups::{Matchup, MatchupData};

pub fn make_matchup_row<'a>(
    title: &'a str,
    matchups: &'a [Matchup],
    champ_data: &'a HashMap<String, ChampionShort>,
) -> Paragraph<'a> {
    Paragraph::new(format!(
        " {}: {}",
        title,
        matchups
            .iter()
            .filter_map(|m| {
                champ_data
                    .get(&m.champion_id.to_string())
                    .map(|c| Cow::from(c.name.clone()))
            })
            .reduce(|mut acc, s| {
                acc.to_mut().push_str(", ");
                acc.to_mut().push_str(&s);
                acc
            })
            .unwrap_or_default()
            .into_owned()
    ))
}

pub fn make_matchups<'a>(
    matchups: &'a MatchupData,
    champ_data: &'a HashMap<String, ChampionShort>,
) -> [impl Widget + 'a; 2] {
    [
        make_matchup_row("Best Matchups", &matchups.best_matchups, champ_data)
            .style(Style::default().fg(Color::Cyan).bold()),
        make_matchup_row("Worst Matchups", &matchups.worst_matchups, champ_data)
            .style(Style::default().fg(Color::Red).bold()),
    ]
}
