use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::components::items;
use crate::components::matchups;
use crate::components::mode_select;
use crate::components::rune_path;
use crate::components::shards;
use crate::components::version_select;
use crate::components::{ability_order, champ_list};
use crate::components::{app_border, search};

use crate::context::{AppContext, State};

const TOO_SMALL_MESSAGE: &str = "Please resize the window to at least 105x28! Ctrl+Q to exit.";
#[allow(clippy::cast_possible_truncation)]
const TOO_SMALL_MESSAGE_LENGTH: u16 = TOO_SMALL_MESSAGE.len() as u16;

pub fn render(frame: &mut Frame, ctx: &AppContext) {
    let frame_size = frame.size();

    let app_border = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(frame_size);

    frame.render_widget(app_border::make(ctx), app_border[0]);

    if frame_size.width <= 105 || frame_size.height <= 28 {
        frame.render_widget(
            Paragraph::new(TOO_SMALL_MESSAGE).style(Style::default().fg(Color::Red).bold()),
            app_border[0].inner(&Margin::new(
                (frame_size.width - TOO_SMALL_MESSAGE_LENGTH) / 2,
                frame_size.height / 2 - 1,
            )),
        );
        return;
    }

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(19), Constraint::Min(0)])
        .margin(1)
        .split(app_border[0].inner(&Margin::new(1, 1)));

    let champion_search_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(main_layout[0]);

    let (champ_list, mut champ_list_state) = champ_list::make(ctx);
    frame.render_stateful_widget(champ_list, champion_search_layout[1], &mut champ_list_state);

    if ctx.champ_list.is_empty() {
        let text = "No results :(";
        let length = 13u16;
        let no_results_text = Paragraph::new(text).style(Style::default().fg(Color::Red));
        let no_results_offset = Rect::new(
            champion_search_layout[1].x + (champion_search_layout[1].width - length) / 2,
            champion_search_layout[1].y + 2,
            length,
            1,
        );
        frame.render_widget(no_results_text, no_results_offset);
    }

    frame.render_widget(search::make(ctx), champion_search_layout[0]);

    let overview_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // champ name
            Constraint::Length(6), // primary / secondary runes
            Constraint::Length(6), // shards / ability order
            Constraint::Length(8), // items
            Constraint::Length(1), // best matchups
            Constraint::Length(1), // worst matchups
            Constraint::Min(0),    // rest
        ])
        .split(main_layout[1]);
    let rune_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(overview_layout[1]);
    let shard_ability_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(overview_layout[2]);

    frame.render_widget(rune_path::make_placeholder(), rune_split[1]);
    frame.render_widget(rune_path::make_placeholder(), rune_split[0]);
    frame.render_widget(shards::make_placeholder(), shard_ability_split[0]);
    frame.render_widget(ability_order::make_placeholder(), shard_ability_split[1]);
    frame.render_widget(items::make_placeholder(), overview_layout[3]);

    if let Some(overview) = &ctx.selected_champ_overview {
        if let Some(selected) = &ctx.selected_champ {
            frame.render_widget(
                Paragraph::new(format!(" Selected: {}", selected.name.clone()))
                    .style(Style::default().fg(Color::Green).bold()),
                overview_layout[0],
            );
        }

        frame.render_widget(
            shards::make(&overview.shards.shard_ids),
            shard_ability_split[0].inner(&Margin::new(1, 1)),
        );

        ability_order::make(shard_ability_split[1].inner(&Margin::new(1, 1)), overview)
            .into_iter()
            .for_each(|(w, r)| frame.render_widget(w, r));

        rune_path::make(overview, &ctx.api.runes)
            .into_iter()
            .zip(rune_split.iter())
            .for_each(|(w, r)| frame.render_widget(w, *r));

        let item_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(overview_layout[3]);

        items::make(overview, &ctx.api.items)
            .into_iter()
            .zip(item_columns.iter())
            .for_each(|(w, r)| frame.render_widget(w, *r));
    }

    if let Some(matchups) = &ctx.selected_champ_matchups {
        let [best, worst] = matchups::make(matchups, &ctx.champ_by_key);
        frame.render_widget(best, overview_layout[4]);
        frame.render_widget(worst, overview_layout[5]);
    }

    if ctx.state == State::ModeSelect {
        let (mode_list, mut mode_list_state, minimum_area) = mode_select::make(ctx);
        let safe_area = main_layout[1].inner(&Margin::new(
            (main_layout[1].width - minimum_area.width) / 2 - 1,
            (main_layout[1].height - minimum_area.height) / 2 - 1,
        ));
        frame.render_widget(Block::new().bg(Color::Black), main_layout[1]);
        frame.render_widget(Clear, safe_area);
        frame.render_stateful_widget(
            mode_list,
            safe_area.inner(&Margin::new(1, 1)),
            &mut mode_list_state,
        );
    }

    if ctx.state == State::VersionSelect {
        let (version_list, mut version_list_state, minimum_area) = version_select::make(ctx);
        let safe_area = main_layout[1].inner(&Margin::new(
            (main_layout[1].width - minimum_area.width) / 2 - 1,
            (main_layout[1].height - minimum_area.height) / 2 - 1,
        ));
        frame.render_widget(Block::new().bg(Color::Black), main_layout[1]);
        frame.render_widget(Clear, safe_area);
        frame.render_stateful_widget(
            version_list,
            safe_area.inner(&Margin::new(1, 1)),
            &mut version_list_state,
        );
    }
}
