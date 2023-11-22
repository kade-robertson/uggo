mod ability_order;
mod app_border;
mod items;
mod matchups;
mod mode_select;
mod rune_path;
mod shards;

use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Clear, List, ListState, Paragraph},
    Frame,
};

use ability_order::{make_ability_order, make_ability_order_placeholder};
use app_border::make_app_border;
use items::{make_item_lists, make_items_placeholder};
use matchups::make_matchups;
use rune_path::{make_rune_paths, make_rune_paths_placeholder};
use shards::{make_shards, make_shards_placeholder};

use crate::context::{AppContext, State};

pub fn render(frame: &mut Frame, ctx: &AppContext) {
    let app_border = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(frame.size());

    frame.render_widget(make_app_border(ctx), app_border[0]);

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(19), Constraint::Min(0)])
        .margin(1)
        .split(app_border[0].inner(&Margin::new(1, 1)));

    let champion_search_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(main_layout[0]);

    frame.render_stateful_widget(
        List::new(ctx.champ_list.clone())
            .block(
                Block::default()
                    .title(" Champions [c] ")
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
        champion_search_layout[1],
        &mut ListState::default().with_selected(ctx.champ_scroll_pos),
    );

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

    frame.render_widget(
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
            ),
        champion_search_layout[0],
    );

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

    frame.render_widget(make_rune_paths_placeholder(), rune_split[0]);
    frame.render_widget(make_rune_paths_placeholder(), rune_split[1]);
    frame.render_widget(make_shards_placeholder(), shard_ability_split[0]);
    frame.render_widget(make_ability_order_placeholder(), shard_ability_split[1]);
    frame.render_widget(make_items_placeholder(), overview_layout[3]);

    if let Some(overview) = &ctx.selected_champ_overview {
        if let Some(selected) = &ctx.selected_champ {
            frame.render_widget(
                Paragraph::new(format!(" Selected: {}", selected.name.clone()))
                    .style(Style::default().fg(Color::Green).bold()),
                overview_layout[0],
            );
        }

        frame.render_widget(
            make_shards(&overview.shards.shard_ids),
            shard_ability_split[0].inner(&Margin::new(1, 1)),
        );

        make_ability_order(shard_ability_split[1].inner(&Margin::new(1, 1)), overview)
            .into_iter()
            .for_each(|(w, r)| frame.render_widget(w, r));

        make_rune_paths(overview, &ctx.api.runes)
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

        make_item_lists(overview, &ctx.api.items)
            .into_iter()
            .zip(item_columns.iter())
            .for_each(|(w, r)| frame.render_widget(w, *r));
    }

    if let Some(matchups) = &ctx.selected_champ_matchups {
        let [best, worst] = make_matchups(matchups, &ctx.champ_by_key);
        frame.render_widget(best, overview_layout[4]);
        frame.render_widget(worst, overview_layout[5]);
    }

    if ctx.state == State::ModeSelect {
        let (mode_list, mut mode_list_state, minimum_area) = mode_select::make_mode_select(ctx);
        let safe_area = main_layout[1].inner(&Margin::new(
            (main_layout[1].width - minimum_area.width) / 2 - 1,
            (main_layout[1].height - minimum_area.height) / 2 - 1,
        ));
        frame.render_widget(Clear, safe_area);
        frame.render_stateful_widget(
            mode_list,
            safe_area.inner(&Margin::new(1, 1)),
            &mut mode_list_state,
        );
    }
}
