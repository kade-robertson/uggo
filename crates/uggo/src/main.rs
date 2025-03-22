#![deny(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use ratatui::crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
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

const HIDE_TARGETS: [&str; 13] = [
    "mio::poll",
    "rustls::client::client_conn",
    "rustls::client::hs",
    "rustls::client::tls13",
    "rustls::conn",
    "rustls::webpki::server_verifier",
    "ureq::pool",
    "ureq::tls::native_tls",
    "ureq::tls::rustls",
    "ureq::unversioned::resolver",
    "ureq::unversioned::transport::tcp",
    "ureq_proto::client::flow",
    "ureq_proto::util",
];

fn main() -> anyhow::Result<()> {
    tui_logger::init_logger(log::LevelFilter::Trace)?;
    tui_logger::set_default_level(log::LevelFilter::Trace);
    for target in HIDE_TARGETS {
        tui_logger::set_level_for_target(target, log::LevelFilter::Error);
    }

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
