use std::collections::HashMap;

use ddragon::models::runes::RuneElement;
use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};
use ugg_types::{overview::OverviewData, rune::RuneExtended};

use crate::util;

fn format_rune_position(rune: &RuneExtended<RuneElement>) -> String {
    let mut position_message = String::new();
    position_message.push('[');
    let mut index = 0;
    while index < rune.siblings {
        if index == rune.index {
            position_message.push('●');
        } else {
            position_message.push('·');
        }
        index += 1;
    }
    position_message.push(']');
    position_message
}

fn rune_color(name: &str) -> Color {
    match name {
        "Precision" => Color::Yellow,
        "Domination" => Color::Red,
        "Sorcery" => Color::Blue,
        "Resolve" => Color::Green,
        "Inspiration" => Color::Cyan,
        _ => Color::Gray,
    }
}

pub fn make_placeholder() -> impl Widget {
    Block::default()
        .white()
        .title(" Rune Paths ")
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
}

fn make_single_rune_path(grouped_runes: &(String, Vec<&RuneExtended<RuneElement>>)) -> impl Widget {
    let secondary_rune_table = Table::new(grouped_runes.1.iter().map(|rune| {
        Row::new(vec![
            Cell::from(Line::from(format_rune_position(rune)).alignment(Alignment::Right)),
            Cell::from(rune.rune.name.clone()),
        ])
    }))
    .style(Style::default().fg(Color::White))
    .column_spacing(1)
    .widths(&[Constraint::Max(6), Constraint::Length(30)])
    .block(
        Block::default()
            .white()
            .title(format!(" ● {} ", grouped_runes.0))
            .title_style(Style::default().fg(rune_color(&grouped_runes.0)).bold())
            .borders(Borders::ALL),
    );
    secondary_rune_table
}

pub fn make(
    overview: &OverviewData,
    runes: &HashMap<i64, RuneExtended<RuneElement>>,
) -> [impl Widget; 2] {
    let grouped_runes = util::group_runes(&overview.runes.rune_ids, runes);
    [
        make_single_rune_path(&grouped_runes[0]),
        make_single_rune_path(&grouped_runes[1]),
    ]
}
