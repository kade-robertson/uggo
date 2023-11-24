use crossterm::event::{Event, KeyEvent};
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};
use tui_input::backend::crossterm::EventHandler;

use crate::context::{AppContext, State};

pub fn make<'a>(ctx: &'a AppContext) -> impl Widget + 'a {
    Paragraph::new(ctx.input.value())
        .style(match ctx.state {
            State::TextInput => Style::default().fg(Color::Green),
            _ => Style::default().fg(Color::White),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Search [s] ")
                .title_style(Style::default().fg(Color::White).bold()),
        )
}

impl AppContext<'_> {
    pub fn on_search_submit(&mut self) {
        self.state = State::ChampScroll;
        if !self.champ_list.is_empty() {
            self.champ_scroll_pos = Some(0);
        }
        if self.champ_list.len() == 1 {
            if let Some(champ) = self
                .list_indices
                .first()
                .and_then(|p| self.champ_data.iter().find(|(i, _)| i == p))
                .map(|(_, c)| c)
                .cloned()
            {
                self.select_champion(&champ);
            }
        }
    }

    pub fn on_search_backspace(&mut self, key: KeyEvent) {
        self.input.handle_event(&Event::Key(key));
        self.update_champ_list();
    }

    pub fn on_search_keypress(&mut self, key: KeyEvent) {
        if self.input.value().len() < 17 {
            self.input.handle_event(&Event::Key(key));
            self.update_champ_list();
        }
    }
}
