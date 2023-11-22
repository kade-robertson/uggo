use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use ugg_types::mappings::Mode;

use crate::context::AppContext;

#[allow(clippy::cast_possible_truncation)]
pub fn make_mode_select<'a>(ctx: &AppContext) -> (List<'a>, ListState, Rect) {
    let mode_list = List::new(
        Mode::all()
            .iter()
            .map(|m| ListItem::new(m.to_string()).style(Style::default().fg(Color::White)))
            .collect::<Vec<_>>(),
    )
    .style(Style::default().fg(Color::White).not_bold())
    .highlight_style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::ITALIC),
    )
    .highlight_symbol("> ")
    .block(
        Block::default()
            .title(" Game Mode ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
    );

    let mode_list_state = ListState::default().with_selected(ctx.mode_scroll_pos);

    (
        mode_list,
        mode_list_state,
        Rect::new(
            0,
            0,
            Mode::all()
                .iter()
                .map(|s| s.to_string().len())
                .max()
                .unwrap_or_default() as u16
                + 5,
            Mode::all().len() as u16 + 2,
        ),
    )
}
