use crate::context::AppContext;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState},
};

#[allow(clippy::cast_possible_truncation)]
pub fn make_version_select<'a>(ctx: &AppContext) -> (List<'a>, ListState, Rect) {
    let version_list = List::new(
        ctx.api
            .allowed_versions
            .iter()
            .map(|m| ListItem::new(m.ddragon.clone()).style(Style::default().fg(Color::White)))
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
            .title(" Game Version ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
    );

    let version_list_state = ListState::default().with_selected(ctx.version_scroll_pos);

    (version_list, version_list_state, Rect::new(0, 0, 16, 14))
}
