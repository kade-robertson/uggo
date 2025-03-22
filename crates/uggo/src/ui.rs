use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, Paragraph, Widget},
};

use tui_logger;
use ugg_types::{mappings::Mode, overview::Overview};

use crate::components::{
    ability_order, app_border, build_select, champ_list, champ_name, items, matchups, mode_select,
    region_select, role_select, rune_path, search, shards, spells, version_select,
};

use crate::context::{AppContext, State};

const TOO_SMALL_MESSAGE: &str = "Please resize the window to at least 105x28! Ctrl+Q to exit.";
#[allow(clippy::cast_possible_truncation)]
const TOO_SMALL_MESSAGE_LENGTH: u16 = TOO_SMALL_MESSAGE.len() as u16;

macro_rules! show_list_popup {
    ($frame:expr,$ui:expr,$layout:expr) => {
        let (list, mut list_state, minimum_area) = $ui;
        let safe_area = $layout.inner(Margin::new(
            ($layout.width - minimum_area.width) / 2 - 1,
            ($layout.height - minimum_area.height) / 2 - 1,
        ));
        $frame.render_widget(Block::new().bg(Color::Black), $layout);
        $frame.render_widget(Clear, safe_area);
        $frame.render_stateful_widget(list, safe_area.inner(Margin::new(1, 1)), &mut list_state);
    };
}

fn render_default_overview(frame: &mut Frame, ctx: &AppContext, main_layout: Rect) {
    if ctx.mode == Mode::Arena {
        return;
    }

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
        .split(main_layout);
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
                champ_name::make(ctx, overview, selected),
                overview_layout[0],
            );
        }

        let item_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(overview_layout[3]);

        if let Overview::Default(d) = overview {
            frame.render_widget(
                shards::make(&d.shards.shard_ids),
                shard_ability_split[0].inner(Margin::new(1, 1)),
            );

            frame.render_widget(
                Paragraph::new(spells::make(ctx, &d.summoner_spells.spell_ids)),
                Rect::new(
                    shard_ability_split[0].x + 1,
                    shard_ability_split[0].y + 4,
                    shard_ability_split[0].width - 1,
                    1,
                ),
            );

            rune_path::make(d, &ctx.api.runes)
                .into_iter()
                .zip(rune_split.iter())
                .for_each(|(w, r)| frame.render_widget(w, *r));

            items::make_default(d, &ctx.api.items)
                .into_iter()
                .zip(item_columns.iter())
                .for_each(|(w, r)| frame.render_widget(w, *r));
        };

        ability_order::make(shard_ability_split[1].inner(Margin::new(1, 1)), overview)
            .into_iter()
            .for_each(|(w, r)| frame.render_widget(w, r));
    }

    if let Some(matchups) = &ctx.selected_champ_matchups {
        let [best, worst] = matchups::make(matchups, &ctx.champ_by_key);
        frame.render_widget(best, overview_layout[4]);
        frame.render_widget(worst, overview_layout[5]);
    }
}

fn render_arena_overview(frame: &mut Frame, ctx: &AppContext, main_layout: Rect) {
    if ctx.mode != Mode::Arena {
        return;
    }

    let overview_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // champ name
            Constraint::Length(5),  // non-prismatic items
            Constraint::Length(12), // prismatic items
            Constraint::Min(0),     // rest
        ])
        .split(main_layout);

    frame.render_widget(items::make_placeholder(), overview_layout[1]);

    if let Some(overview) = &ctx.selected_champ_overview {
        if let Some(selected) = &ctx.selected_champ {
            frame.render_widget(
                champ_name::make(ctx, overview, selected),
                overview_layout[0],
            );
        }

        let item_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
            ])
            .split(overview_layout[1]);

        if let Overview::Arena(d) = overview {
            let [regular @ .., prismatic] = items::make_arena(d, &ctx.api.items);

            regular
                .into_iter()
                .zip(item_columns.iter())
                .for_each(|(w, r)| frame.render_widget(w, *r));

            frame.render_widget(prismatic, overview_layout[2]);
        }
    }
}

pub fn render(frame: &mut Frame, ctx: &AppContext) {
    let frame_size = frame.area();

    let app_border = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(frame_size);

    frame.render_widget(app_border::make(ctx), app_border[0]);

    if frame_size.width <= 105 || frame_size.height <= 28 {
        frame.render_widget(
            Paragraph::new(TOO_SMALL_MESSAGE).style(Style::default().fg(Color::Red).bold()),
            app_border[0].inner(Margin::new(
                (frame_size.width - TOO_SMALL_MESSAGE_LENGTH) / 2,
                frame_size.height / 2 - 1,
            )),
        );
        return;
    }

    if ctx.state == State::Logger {
        let container = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0)])
            .margin(1)
            .split(app_border[0].inner(Margin::new(1, 1)));
        let logger = tui_logger::TuiLoggerSmartWidget::default().state(&ctx.logger_state);
        frame.render_widget(logger, container[0]);
        return;
    }

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(19), Constraint::Min(0)])
        .margin(1)
        .split(app_border[0].inner(Margin::new(1, 1)));

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

    match ctx.mode {
        Mode::Arena => render_arena_overview(frame, ctx, main_layout[1]),
        _ => render_default_overview(frame, ctx, main_layout[1]),
    }

    if ctx.state == State::ModeSelect {
        show_list_popup!(frame, mode_select::make(ctx), main_layout[1]);
    }

    if ctx.state == State::VersionSelect {
        show_list_popup!(frame, version_select::make(ctx), main_layout[1]);
    }

    if ctx.state == State::RegionSelect {
        show_list_popup!(frame, region_select::make(ctx), main_layout[1]);
    }

    if ctx.state == State::RoleSelect {
        show_list_popup!(frame, role_select::make(ctx), main_layout[1]);
    }

    if ctx.state == State::BuildSelect {
        show_list_popup!(frame, build_select::make(ctx), main_layout[1]);
    }

    if ctx.state == State::HelpMenu {
        let (help_menu, minimum_area) = crate::components::help_menu::make();
        let safe_area = main_layout[1].inner(Margin::new(
            (main_layout[1].width - minimum_area.width) / 2 - 1,
            (main_layout[1].height - minimum_area.height) / 2 - 1,
        ));
        frame.render_widget(Block::new().bg(Color::Black), main_layout[1]);
        frame.render_widget(Clear, safe_area);
        frame.render_widget(help_menu, safe_area.inner(Margin::new(1, 1)));
    }
}
