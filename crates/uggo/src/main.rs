#![deny(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io::{self, stdout};
use tui_input::{backend::crossterm::EventHandler, Input};
use uggo_config::Config;
use uggo_ugg_api::{UggApi, UggApiBuilder};

enum State {
    Initial,
    TextInput,
    ChampScroll,
}

struct AppContext<'a> {
    api: UggApi,
    state: State,
    scroll_pos: Option<usize>,
    champ_list: Vec<ListItem<'a>>,
    input: Input,
}

fn update_champ_list(ctx: &mut AppContext) {
    let mut ordered_champ_names = ctx
        .api
        .champ_data
        .values()
        .map(|c| c.name.clone())
        .collect::<Vec<_>>();
    ordered_champ_names.sort();

    ctx.champ_list = ordered_champ_names
        .iter()
        .filter(|c| c.to_lowercase().contains(&ctx.input.value().to_lowercase()))
        .map(|c| ListItem::new(c.clone()))
        .collect::<Vec<_>>();
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let config = Config::new()?;
    let api = UggApiBuilder::new().cache_dir(config.cache()).build()?;

    let mut app_context = AppContext {
        api,
        state: State::Initial,
        scroll_pos: None,
        champ_list: Vec::new(),
        input: Input::default(),
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

fn handle_events(ctx: &mut AppContext) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match ctx.state {
                State::Initial => {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                        ctx.state = State::TextInput;
                    }
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        return Ok(true);
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
                    }
                    _ => {
                        ctx.input.handle_event(&Event::Key(key));
                        update_champ_list(ctx);
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
                    _ => {}
                },
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, ctx: &AppContext) {
    let outer_block = Block::default()
        .title(format!(" uggo v{} ", env!("CARGO_PKG_VERSION")))
        .title_style(Style::default().bold())
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .magenta();
    let app_border = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(frame.size());
    frame.render_widget(outer_block, app_border[0]);

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
    frame.render_widget(
        Block::default()
            .white()
            .title("Right")
            .borders(Borders::ALL),
        main_layout[1],
    );

    let champion_search_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(main_layout[0]);
    frame.render_stateful_widget(
        List::new(ctx.champ_list.clone())
            .block(
                Block::default()
                    .title("Champion List")
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
            .block(Block::default().borders(Borders::ALL).title("Search")),
        champion_search_layout[0],
    );
}
