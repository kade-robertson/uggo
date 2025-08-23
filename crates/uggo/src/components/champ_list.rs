use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListState},
};

use crate::context::{AppContext, State};

pub fn make<'a>(ctx: &'a AppContext) -> (List<'a>, ListState) {
    (
        List::new(ctx.champ_list.clone())
            .block(
                Block::default()
                    .title(" Champions ")
                    .style(Style::default().fg(Color::White).bold())
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White).not_bold())
            .highlight_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::ITALIC),
            )
            .highlight_symbol("> "),
        ListState::default().with_selected(ctx.champ_scroll_pos),
    )
}

impl AppContext<'_> {
    pub fn next_champ(&mut self) {
        if let Some(pos) = self.champ_scroll_pos
            && pos < self.champ_list.len() - 1
        {
            self.champ_scroll_pos = Some(pos + 1);
        }
    }

    pub fn prev_champ(&mut self) {
        if let Some(pos) = self.champ_scroll_pos
            && pos > 0
        {
            self.champ_scroll_pos = Some(pos - 1);
        }
    }

    pub fn select_champ(&mut self) {
        if let Some(champ) = self
            .champ_scroll_pos
            .and_then(|p| self.list_indices.get(p))
            .and_then(|p| self.champ_data.iter().find(|(i, _)| i == p))
            .map(|(_, c)| c)
            .cloned()
        {
            self.select_champion(&champ);
        }
    }

    pub fn go_to_search(&mut self) {
        self.state = State::TextInput;
        self.update_champ_list();
    }
}
