#![deny(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]

use ddragon::models::champions::ChampionShort;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table,
    },
};
use std::{
    borrow::{BorrowMut, Cow},
    io::{self, stdout},
};
use styling::{format_ability_level_order, format_rune_position};
use tui_input::{backend::crossterm::EventHandler, Input};
use ugg_types::{
    client_runepage::NewRunePage,
    mappings::{Mode, Region, Role},
    matchups::MatchupData,
    overview::OverviewData,
};
use uggo_config::Config;
use uggo_lol_client::LOLClientAPI;
use uggo_ugg_api::{UggApi, UggApiBuilder};
use util::{group_runes, process_shards};

mod styling;
mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Initial,
    TextInput,
    ChampScroll,
    ChampSelected,
}

struct AppContext<'a> {
    api: UggApi,
    client_api: Option<LOLClientAPI>,
    state: State,
    scroll_pos: Option<usize>,
    champ_data: Vec<(usize, ChampionShort)>,
    list_indices: Vec<usize>,
    champ_list: Vec<ListItem<'a>>,
    selected_champ: Option<ChampionShort>,
    selected_champ_overview: Option<OverviewData>,
    selected_champ_matchups: Option<MatchupData>,
    max_item_length: usize,
    items: Vec<String>,
    mode: Mode,
    input: Input,
}

fn update_champ_list(ctx: &mut AppContext) {
    (ctx.list_indices, ctx.champ_list) = ctx
        .champ_data
        .iter()
        .filter(|(_, c)| {
            c.name
                .to_lowercase()
                .contains(&ctx.input.value().to_lowercase())
        })
        .map(|(i, c)| (i, ListItem::new(c.name.clone())))
        .unzip();
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let config = Config::new()?;
    let api = UggApiBuilder::new()
        .version("13.22.1")
        .cache_dir(config.cache())
        .build()?;

    let mut ordered_champ_data = api
        .champ_data
        .values()
        .enumerate()
        .map(|(i, c)| (i, c.clone()))
        .collect::<Vec<_>>();
    ordered_champ_data.sort_by(|(_, a), (_, b)| a.name.cmp(&b.name));

    let mut ordered_item_names = api
        .items
        .values()
        .map(|i| i.name.clone())
        .collect::<Vec<_>>();
    ordered_item_names.sort_by_key(std::string::String::len);
    ordered_item_names.reverse();

    let max_item_length = ordered_item_names
        .first()
        .map(std::string::String::len)
        .unwrap_or_default();

    let mut app_context = AppContext {
        api,
        client_api: LOLClientAPI::new().ok(),
        state: State::Initial,
        scroll_pos: None,
        champ_data: ordered_champ_data,
        list_indices: Vec::new(),
        champ_list: Vec::new(),
        input: Input::default(),
        selected_champ: None,
        selected_champ_overview: None,
        selected_champ_matchups: None,
        max_item_length,
        items: ordered_item_names,
        mode: Mode::Normal,
    };
    update_champ_list(&mut app_context);

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &app_context))?;
        should_quit = handle_events(&mut app_context)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn select_champion(ctx: &mut AppContext, champ: &ChampionShort) {
    ctx.scroll_pos = None;
    ctx.selected_champ = Some(champ.clone());
    ctx.selected_champ_overview = ctx
        .api
        .get_stats(champ, Role::Automatic, Region::World, ctx.mode)
        .map(|v| *v)
        .ok();
    if ctx.mode == Mode::ARAM {
        ctx.selected_champ_matchups = None;
    } else {
        ctx.selected_champ_matchups = ctx
            .api
            .get_matchups(champ, Role::Automatic, Region::World, ctx.mode)
            .map(|v| *v)
            .ok();
    }

    if let Some(ref overview) = ctx.selected_champ_overview {
        if let Some(ref api) = ctx.client_api {
            if let Some(data) = api.get_current_rune_page() {
                let (primary_style_id, sub_style_id, selected_perk_ids) = util::generate_perk_array(
                    &util::group_runes(&overview.runes.rune_ids, &ctx.api.runes),
                    &overview.shards.shard_ids,
                );
                api.update_rune_page(
                    data.id,
                    &NewRunePage {
                        name: format!("uggo: {}, {}", &champ.name, ctx.mode),
                        primary_style_id,
                        sub_style_id,
                        selected_perk_ids,
                    },
                );
            }
        }
    }

    ctx.state = State::ChampSelected;
}

fn handle_events(ctx: &mut AppContext) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press
                && key.code == KeyCode::Char('q')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                return Ok(true);
            }
            if ctx.state != State::TextInput
                && key.kind == event::KeyEventKind::Press
                && key.code == KeyCode::Char('m')
            {
                // Cycle through all modes.
                ctx.mode = match ctx.mode {
                    Mode::Normal => Mode::ARAM,
                    Mode::ARAM => Mode::OneForAll,
                    Mode::OneForAll => Mode::URF,
                    Mode::URF => Mode::ARURF,
                    Mode::ARURF => Mode::NexusBlitz,
                    Mode::NexusBlitz => Mode::Normal,
                };
                let selected = ctx.selected_champ.clone();
                if let Some(champ) = selected {
                    select_champion(ctx, &champ);
                }
                return Ok(false);
            }
            match ctx.state {
                State::ChampSelected | State::Initial => {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('s') {
                        ctx.state = State::TextInput;
                    }
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('c') {
                        ctx.state = State::ChampScroll;
                        if !ctx.champ_list.is_empty() {
                            ctx.scroll_pos = Some(0);
                        }
                    }
                }
                State::TextInput => match key.code {
                    KeyCode::Esc => {
                        ctx.state = State::Initial;
                        ctx.scroll_pos = None;
                    }
                    KeyCode::Enter => {
                        ctx.state = State::ChampScroll;
                        if !ctx.champ_list.is_empty() {
                            ctx.scroll_pos = Some(0);
                        }
                        if ctx.champ_list.len() == 1 {
                            if let Some(champ) = ctx
                                .list_indices
                                .first()
                                .and_then(|p| ctx.champ_data.iter().find(|(i, _)| i == p))
                                .map(|(_, c)| c)
                                .cloned()
                            {
                                select_champion(ctx, &champ);
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        ctx.input.handle_event(&Event::Key(key));
                        update_champ_list(ctx);
                    }
                    _ => {
                        if ctx.input.value().len() < 17 {
                            ctx.input.handle_event(&Event::Key(key));
                            update_champ_list(ctx);
                        }
                    }
                },
                State::ChampScroll => match key.code {
                    KeyCode::Esc => {
                        ctx.state = State::Initial;
                        ctx.scroll_pos = None;
                    }
                    KeyCode::Up => {
                        if let Some(pos) = ctx.scroll_pos {
                            if pos > 0 {
                                ctx.scroll_pos = Some(pos - 1);
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Some(pos) = ctx.scroll_pos {
                            if pos < ctx.champ_list.len() - 1 {
                                ctx.scroll_pos = Some(pos + 1);
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(champ) = ctx
                            .scroll_pos
                            .and_then(|p| ctx.list_indices.get(p))
                            .and_then(|p| ctx.champ_data.iter().find(|(i, _)| i == p))
                            .map(|(_, c)| c)
                            .cloned()
                        {
                            select_champion(ctx, &champ);
                        }
                    }
                    KeyCode::Char('s') => {
                        ctx.state = State::TextInput;
                        ctx.scroll_pos = None;
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(false)
}

fn make_app_border(frame: &mut Frame, ctx: &AppContext) {
    let outer_block = Block::default()
        .title(
            Title::from(format!(" uggo v{} ", env!("CARGO_PKG_VERSION")))
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from(" [Esc: Back] [Enter: Commit] [m: Cycle Mode] [Ctrl + q: Exit] ")
                .position(Position::Bottom)
                .alignment(Alignment::Left),
        )
        .title(
            Title::from(format!(" [Mode: {}] ", ctx.mode))
                .position(Position::Bottom)
                .alignment(Alignment::Right),
        )
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
        .magenta();

    let app_border = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(frame.size());

    frame.render_widget(outer_block, app_border[0]);
}

fn make_champ_overview(frame: &mut Frame, bounds: Rect, overview: &OverviewData) {
    // Draw Q |
    frame.render_widget(
        Paragraph::new("Q│ "),
        Rect::new(bounds.left(), bounds.top(), 4, 1),
    );
    // Draw Q abilities
    frame.render_widget(
        Paragraph::new(format_ability_level_order(
            &overview.abilities.ability_order,
            'Q',
        ))
        .style(Style::default().fg(Color::Cyan).bold()),
        Rect::new(bounds.left() + 3, bounds.top(), 36, 4),
    );
    // Draw W |
    frame.render_widget(
        Paragraph::new("W│ "),
        Rect::new(bounds.left(), bounds.top() + 1, 4, 1),
    );
    // Draw W abilities
    frame.render_widget(
        Paragraph::new(format_ability_level_order(
            &overview.abilities.ability_order,
            'W',
        ))
        .style(Style::default().fg(Color::Yellow).bold()),
        Rect::new(bounds.left() + 3, bounds.top() + 1, 36, 4),
    );

    // Draw E |
    frame.render_widget(
        Paragraph::new("E│ "),
        Rect::new(bounds.left(), bounds.top() + 2, 4, 1),
    );
    // Draw E abilities
    frame.render_widget(
        Paragraph::new(format_ability_level_order(
            &overview.abilities.ability_order,
            'E',
        ))
        .style(Style::default().fg(Color::Green).bold()),
        Rect::new(bounds.left() + 3, bounds.top() + 2, 36, 1),
    );
    // Draw R |
    frame.render_widget(
        Paragraph::new("R│ "),
        Rect::new(bounds.left(), bounds.top() + 3, 4, 1),
    );
    // Draw R abilities
    frame.render_widget(
        Paragraph::new(format_ability_level_order(
            &overview.abilities.ability_order,
            'R',
        ))
        .style(Style::default().fg(Color::Red).bold()),
        Rect::new(bounds.left() + 3, bounds.top() + 3, 36, 1),
    );
    // Reset style
    frame.render_widget(
        Paragraph::new("").style(Style::default().white()),
        Rect::new(bounds.left(), bounds.bottom(), bounds.width, bounds.height),
    );
}

fn ui(frame: &mut Frame, ctx: &AppContext) {
    make_app_border(frame, ctx);

    let main_layout_size = Rect::new(
        frame.size().x + 1,
        frame.size().y + 1,
        frame.size().width - 1,
        frame.size().height - 1,
    );
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(19), Constraint::Min(0)])
        .margin(1)
        .split(main_layout_size);

    let overview_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // champ name
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

    frame.render_widget(
        Block::default()
            .white()
            .title(" Rune Path ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
        rune_split[0],
    );
    frame.render_widget(
        Block::default()
            .white()
            .title(" Rune Path ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
        rune_split[1],
    );
    frame.render_widget(
        Block::default()
            .white()
            .title(" Shards ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
        shard_ability_split[0],
    );
    frame.render_widget(
        Block::default()
            .white()
            .title(" Ability Order ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
        shard_ability_split[1],
    );
    frame.render_widget(
        Block::default()
            .white()
            .title(" Items ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL),
        overview_layout[3],
    );
    if let Some(overview) = &ctx.selected_champ_overview {
        if let Some(selected) = &ctx.selected_champ {
            frame.render_widget(
                Paragraph::new(format!(" Selected: {}", selected.name.clone()))
                    .style(Style::default().fg(Color::Green).bold()),
                overview_layout[0],
            );
        }

        frame.render_widget(
            Paragraph::new(process_shards(&overview.shards.shard_ids)),
            shard_ability_split[0].inner(&Margin::new(1, 1)),
        );

        make_champ_overview(
            frame,
            shard_ability_split[1].inner(&Margin::new(1, 1)),
            overview,
        );

        let grouped_runes = group_runes(&overview.runes.rune_ids, &ctx.api.runes);

        let primary_rune_table = Table::new(grouped_runes[0].1.iter().map(|rune| {
            Row::new(vec![
                Cell::from(format_rune_position(rune)),
                Cell::from(rune.rune.name.clone()),
            ])
        }))
        .style(Style::default().fg(Color::White))
        .column_spacing(1)
        .widths(&[Constraint::Max(6), Constraint::Length(30)]);

        let secondary_rune_table = Table::new(grouped_runes[1].1.iter().map(|rune| {
            Row::new(vec![
                Cell::from(format_rune_position(rune)),
                Cell::from(rune.rune.name.clone()),
            ])
        }))
        .style(Style::default().fg(Color::White))
        .column_spacing(1)
        .widths(&[Constraint::Max(6), Constraint::Length(30)]);

        frame.render_widget(
            primary_rune_table.block(
                Block::default()
                    .white()
                    .title(format!(" {} ", grouped_runes[0].0))
                    .title_style(Style::default().bold())
                    .borders(Borders::ALL),
            ),
            rune_split[0],
        );
        frame.render_widget(
            secondary_rune_table.block(
                Block::default()
                    .white()
                    .title(format!(" {} ", grouped_runes[1].0))
                    .title_style(Style::default().bold())
                    .borders(Borders::ALL),
            ),
            rune_split[1],
        );

        let item_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(overview_layout[3]);

        frame.render_widget(
            List::new(
                overview
                    .starting_items
                    .item_ids
                    .iter()
                    .filter_map(|i| {
                        ctx.api
                            .items
                            .get(&i.to_string())
                            .map(|it| ListItem::new(it.name.clone()))
                    })
                    .collect::<Vec<_>>(),
            )
            .block(
                Block::default()
                    .white()
                    .title(" Starting Items ")
                    .title_style(Style::default().bold())
                    .borders(Borders::ALL),
            ),
            item_columns[0],
        );

        frame.render_widget(
            List::new(
                overview
                    .item_4_options
                    .iter()
                    .filter_map(|i| {
                        ctx.api
                            .items
                            .get(&i.id.to_string())
                            .map(|it| ListItem::new(it.name.clone()))
                    })
                    .collect::<Vec<_>>(),
            )
            .block(
                Block::default()
                    .white()
                    .title(" 4th Item ")
                    .title_style(Style::default().bold())
                    .borders(Borders::ALL),
            ),
            item_columns[1],
        );

        frame.render_widget(
            List::new(
                overview
                    .item_5_options
                    .iter()
                    .filter_map(|i| {
                        ctx.api
                            .items
                            .get(&i.id.to_string())
                            .map(|it| ListItem::new(it.name.clone()))
                    })
                    .collect::<Vec<_>>(),
            )
            .block(
                Block::default()
                    .white()
                    .title(" 5th Item ")
                    .title_style(Style::default().bold())
                    .borders(Borders::ALL),
            ),
            item_columns[2],
        );

        frame.render_widget(
            List::new(
                overview
                    .item_6_options
                    .iter()
                    .filter_map(|i| {
                        ctx.api
                            .items
                            .get(&i.id.to_string())
                            .map(|it| ListItem::new(it.name.clone()))
                    })
                    .collect::<Vec<_>>(),
            )
            .block(
                Block::default()
                    .white()
                    .title(" 6th Item ")
                    .title_style(Style::default().bold())
                    .borders(Borders::ALL),
            ),
            item_columns[3],
        );
    }

    if let Some(matchups) = &ctx.selected_champ_matchups {
        frame.render_widget(
            Paragraph::new(format!(
                " Best Matchups: {}",
                matchups
                    .best_matchups
                    .iter()
                    .filter_map(|m| {
                        ctx.api
                            .champ_data
                            .values()
                            .find(|c| c.key == m.champion_id.to_string())
                            .map(|c| Cow::from(c.name.clone()))
                    })
                    .reduce(|mut acc, s| {
                        acc.to_mut().push_str(", ");
                        acc.to_mut().push_str(&s);
                        acc
                    })
                    .unwrap_or_default()
                    .into_owned()
            ))
            .style(Style::default().fg(Color::Cyan).bold()),
            overview_layout[4],
        );
        frame.render_widget(
            Paragraph::new(format!(
                " Worst Matchups: {}",
                matchups
                    .worst_matchups
                    .iter()
                    .filter_map(|m| {
                        ctx.api
                            .champ_data
                            .values()
                            .find(|c| c.key == m.champion_id.to_string())
                            .map(|c| Cow::from(c.name.clone()))
                    })
                    .reduce(|mut acc, s| {
                        acc.to_mut().push_str(", ");
                        acc.to_mut().push_str(&s);
                        acc
                    })
                    .unwrap_or_default()
                    .into_owned()
            ))
            .style(Style::default().fg(Color::Red).bold()),
            overview_layout[5],
        );
    }

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
        &mut ListState::default().with_selected(ctx.scroll_pos),
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
}
