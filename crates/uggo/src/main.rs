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
        Block, Borders, List, ListItem, ListState, Paragraph,
    },
};
use std::io::{self, stdout};
use styling::format_ability_level_order;
use tui_input::{backend::crossterm::EventHandler, Input};
use ugg_types::{
    mappings::{Mode, Region, Role},
    matchups::MatchupData,
    overview::OverviewData,
};
use uggo_config::Config;
use uggo_ugg_api::{UggApi, UggApiBuilder};

mod styling;

enum State {
    Initial,
    TextInput,
    ChampScroll,
    ChampSelected,
}

struct AppContext<'a> {
    api: UggApi,
    state: State,
    scroll_pos: Option<usize>,
    champ_data: Vec<(usize, ChampionShort)>,
    list_indices: Vec<usize>,
    champ_list: Vec<ListItem<'a>>,
    selected_champ: Option<ChampionShort>,
    selected_champ_overview: Option<OverviewData>,
    selected_champ_matchups: Option<MatchupData>,
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
    let api = UggApiBuilder::new().cache_dir(config.cache()).build()?;

    let mut ordered_champ_data = api
        .champ_data
        .values()
        .enumerate()
        .map(|(i, c)| (i, c.clone()))
        .collect::<Vec<_>>();
    ordered_champ_data.sort_by(|(_, a), (_, b)| a.name.cmp(&b.name));

    let mut app_context = AppContext {
        api,
        state: State::Initial,
        scroll_pos: None,
        champ_data: ordered_champ_data,
        list_indices: Vec::new(),
        champ_list: Vec::new(),
        input: Input::default(),
        selected_champ: None,
        selected_champ_overview: None,
        selected_champ_matchups: None,
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
        .get_stats(champ, Role::Automatic, Region::World, Mode::Normal)
        .map(|v| *v)
        .ok();
    ctx.selected_champ_matchups = ctx
        .api
        .get_matchups(champ, Role::Automatic, Region::World, Mode::Normal)
        .map(|v| *v)
        .ok();

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

fn make_app_border(frame: &mut Frame) {
    let outer_block = Block::default()
        .title(
            Title::from(format!(" uggo v{} ", env!("CARGO_PKG_VERSION")))
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from(" [Esc: Back] [Enter: Commit] [Ctrl + q: Exit] ")
                .position(Position::Bottom)
                .alignment(Alignment::Left),
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
    make_app_border(frame);

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
            Constraint::Length(6), // ability order
            Constraint::Min(0),    // rest
        ])
        .split(main_layout[1]);
    frame.render_widget(
        Block::default()
            .white()
            .title("Ability Order")
            .borders(Borders::ALL),
        overview_layout[0],
    );
    if let Some(overview) = &ctx.selected_champ_overview {
        make_champ_overview(
            frame,
            overview_layout[0].inner(&Margin::new(1, 1)),
            overview,
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
                    .title("Champion List [c]")
                    .style(match ctx.state {
                        State::ChampScroll => Style::default().fg(Color::White).bold(),
                        _ => Style::default().fg(Color::White),
                    })
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
            .block(Block::default().borders(Borders::ALL).title("Search [s]")),
        champion_search_layout[0],
    );
}
