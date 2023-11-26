#![deny(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::stdout;

#[cfg(debug_assertions)]
use std::time::Instant;

mod components;
mod context;
mod events;
mod transpose;
mod ui;
mod util;

use context::AppContext;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app_context = AppContext::new()?;
    let mut should_quit = false;
    while !should_quit {
        #[cfg(debug_assertions)]
        let start_render = Instant::now();

        terminal.draw(|frame| ui::render(frame, &app_context))?;

        #[cfg(debug_assertions)]
        app_context.set_render_duration(start_render.elapsed());

        should_quit = events::handle_events(&mut app_context)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
