use ddragon::models::champions::ChampionShort;
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Paragraph, Widget},
};
use ugg_types::overview::Overview;

use crate::context::AppContext;

pub fn make<'a>(
    ctx: &'a AppContext,
    overview: &'a Overview,
    selected: &'a ChampionShort,
) -> impl Widget + 'a {
    let champ_name = selected.name.clone();
    let (selected_text, color) = if overview.low_sample_size() {
        (
            format!(
                " Selected: {champ_name}, Role: {}, Build: {}\n ⚠️ Warning: Low Sample Size",
                ctx.selected_champ_role.unwrap_or(ctx.role),
                ctx.build
            ),
            Color::Yellow,
        )
    } else {
        (
            format!(
                " Selected: {champ_name}, Role: {}, Build: {}",
                ctx.selected_champ_role.unwrap_or(ctx.role),
                ctx.build
            ),
            Color::Green,
        )
    };

    Paragraph::new(selected_text).style(Style::default().fg(color).bold())
}
