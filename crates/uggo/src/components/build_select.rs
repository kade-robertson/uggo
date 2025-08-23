use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use ugg_types::mappings::Build;

use crate::context::{AppContext, State};

#[allow(clippy::cast_possible_truncation)]
pub fn make<'a>(ctx: &AppContext) -> (List<'a>, ListState, Rect) {
    let overview_kind_list = List::new(
        Build::all()
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
            .title(" Overview Kind ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
    );

    let overview_kind_list_state = ListState::default().with_selected(ctx.build_scroll_pos);

    (
        overview_kind_list,
        overview_kind_list_state,
        Rect::new(
            0,
            0,
            Build::all()
                .iter()
                .map(|s| s.to_string().len())
                .max()
                .unwrap_or_default() as u16
                + 5,
            Build::all().len() as u16 + 1,
        ),
    )
}

impl AppContext<'_> {
    pub fn next_build(&mut self) {
        if let Some(pos) = self.build_scroll_pos
            && pos < Build::all().len() - 1
        {
            self.build_scroll_pos = Some(pos + 1);
        }
    }

    pub fn prev_build(&mut self) {
        if let Some(pos) = self.build_scroll_pos
            && pos > 0
        {
            self.build_scroll_pos = Some(pos - 1);
        }
    }

    pub fn select_build(&mut self) {
        if let Some(overview_kind) = self.build_scroll_pos.and_then(|p| Build::all().get(p)) {
            self.build = *overview_kind;
            self.state = State::Initial;
            if let Some(champ) = self.selected_champ.clone() {
                self.select_champion(&champ);
                self.state = State::ChampSelected;
            }
        }
    }
}
