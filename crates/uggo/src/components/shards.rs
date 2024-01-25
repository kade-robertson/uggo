use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};

const fn shard_color(shard: i64) -> Color {
    match shard {
        5001 | 5011 => Color::Green,
        5002 => Color::Red,
        5003 | 5013 => Color::Magenta,
        5005 => Color::Yellow,
        5008 => Color::Blue,
        5010 => Color::White,
        _ => Color::Gray,
    }
}

const fn shard_text(id: i64) -> &'static str {
    match id {
        5001 => "+10-180 Health",
        5002 => "+6 Armor",
        5003 => "+8 Magic Resist",
        5005 => "+10% Attack Speed",
        5007 => "+8 Ability Haste",
        5008 => "+9 Adaptive Force",
        5010 => "+2% Move Speed",
        5011 => "65 Health",
        5013 => "+10% Tenacity and Slow Resist",
        _ => "Unknown",
    }
}

pub fn make_placeholder() -> impl Widget {
    Block::default()
        .white()
        .title(" Shards & Spells ")
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
}

fn make_shard_row(name: &str, shard: i64) -> Row {
    Row::new(vec![
        Cell::from(Line::from(name).alignment(Alignment::Right)),
        Cell::from(Text::styled(
            "●",
            Style::default().fg(shard_color(shard)).bold(),
        )),
        Cell::from(shard_text(shard)),
    ])
}

pub fn make(shards: &[i64]) -> impl Widget {
    Table::new(vec![
        make_shard_row("Offense", shards[0]),
        make_shard_row("Flex", shards[1]),
        make_shard_row("Defense", shards[2]),
    ])
    .column_spacing(1)
    .widths(&[
        Constraint::Length(7),
        Constraint::Length(1),
        Constraint::Length(20),
    ])
}
