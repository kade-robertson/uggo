use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use ugg_types::mappings::Role;

use crate::context::{AppContext, State};

#[allow(clippy::cast_possible_truncation)]
pub fn make<'a>(ctx: &AppContext) -> (List<'a>, ListState, Rect) {
    let role_list = List::new(
        Role::all()
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
            .title(" Role ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
    );

    let role_list_state = ListState::default().with_selected(ctx.role_scroll_pos);

    (
        role_list,
        role_list_state,
        Rect::new(
            0,
            0,
            Role::all()
                .iter()
                .map(|s| s.to_string().len())
                .max()
                .unwrap_or_default() as u16
                + 5,
            Role::all().len() as u16 + 2,
        ),
    )
}

impl AppContext<'_> {
    pub fn next_role(&mut self) {
        if let Some(pos) = self.role_scroll_pos {
            if pos < Role::all().len() - 1 {
                self.role_scroll_pos = Some(pos + 1);
            }
        }
    }

    pub fn prev_role(&mut self) {
        if let Some(pos) = self.role_scroll_pos {
            if pos > 0 {
                self.role_scroll_pos = Some(pos - 1);
            }
        }
    }

    pub fn select_role(&mut self) {
        if let Some(role) = self.role_scroll_pos.and_then(|p| Role::all().get(p)) {
            self.role = *role;
            self.state = State::Initial;
            if let Some(champ) = self.selected_champ.clone() {
                self.select_champion(&champ);
                self.state = State::ChampSelected;
            }
        }
    }

    pub fn match_pos_to_role(&mut self) {
        self.role_scroll_pos = Role::all().iter().position(|r| r == &self.role);
    }
}
