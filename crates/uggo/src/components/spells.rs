use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::context::AppContext;

fn spell_dot(spell: &str) -> Span {
    Span::styled(
        " ● ",
        Style::default().fg(match spell {
            "Flash" | "Barrier" | "Exhaust" => Color::Yellow,
            "Ghost" | "Clarity" => Color::Blue,
            "Cleanse" | "Dash" => Color::Cyan,
            "Teleport" => Color::Magenta,
            "Heal" => Color::Green,
            "Smite" | "Ignite" => Color::Red,
            "Mark" => Color::White,
            _ => Color::Gray,
        }),
    )
}

pub fn make<'a>(ctx: &'a AppContext, spells: &'a [i64]) -> Line<'a> {
    let spell_1 = ctx
        .api
        .summoner_spells
        .get(&spells[0])
        .map_or("Unknown", |s| s.as_str());
    let spell_2 = ctx
        .api
        .summoner_spells
        .get(&spells[1])
        .map_or("Unknown", |s| s.as_str());

    Line::from(vec![
        Span::styled("Spells:", Style::default().fg(Color::White)),
        spell_dot(spell_1),
        Span::styled(spell_1, Style::default().fg(Color::White)),
        Span::styled(" +", Style::default().fg(Color::White)),
        spell_dot(spell_2),
        Span::styled(spell_2, Style::default().fg(Color::White)),
    ])
}
