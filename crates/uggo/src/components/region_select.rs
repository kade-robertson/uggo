use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use ugg_types::mappings::Region;

use crate::context::{AppContext, State};

#[allow(clippy::cast_possible_truncation)]
pub fn make<'a>(ctx: &AppContext) -> (List<'a>, ListState, Rect) {
    let region_list = List::new(
        Region::all()
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
            .title(" Region ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
    );

    let mode_list_state = ListState::default().with_selected(ctx.region_scroll_pos);

    (
        region_list,
        mode_list_state,
        Rect::new(
            0,
            0,
            Region::all()
                .iter()
                .map(|s| s.to_string().len())
                .max()
                .unwrap_or_default() as u16
                + 5,
            Region::all().len() as u16 + 2,
        ),
    )
}

impl AppContext<'_> {
    pub fn next_region(&mut self) {
        if let Some(pos) = self.region_scroll_pos
            && pos < Region::all().len() - 1
        {
            self.region_scroll_pos = Some(pos + 1);
        }
    }

    pub fn prev_region(&mut self) {
        if let Some(pos) = self.region_scroll_pos
            && pos > 0
        {
            self.region_scroll_pos = Some(pos - 1);
        }
    }

    pub fn select_region(&mut self) {
        if let Some(region) = self.region_scroll_pos.and_then(|p| Region::all().get(p)) {
            self.region = *region;
            self.state = State::Initial;
            if let Some(champ) = self.selected_champ.clone() {
                self.select_champion(&champ);
                self.state = State::ChampSelected;
            }
        }
    }

    pub fn match_pos_to_region(&mut self) {
        self.region_scroll_pos = Region::all().iter().position(|r| r == &self.region);
    }
}
