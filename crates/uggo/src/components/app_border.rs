use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    widgets::{
        block::{Position, Title},
        Block, Borders, Widget,
    },
};

use crate::context::AppContext;

#[allow(clippy::cast_precision_loss)]
#[cfg(debug_assertions)]
fn make_bottom_right_title<'a>(ctx: &'a AppContext) -> Title<'a> {
    Title::from(format!(
        " [Mode: {}] [Patch: {}] [Region: {}] [Render: {:.2}ms] ",
        ctx.mode,
        ctx.version,
        ctx.region,
        ctx.last_render_duration
            .map_or(0.0, |d| d.as_micros() as f64 / 1000.0)
    ))
    .position(Position::Bottom)
    .alignment(Alignment::Right)
}

#[allow(clippy::cast_precision_loss)]
#[cfg(not(debug_assertions))]
fn make_bottom_right_title<'a>(ctx: &'a AppContext) -> Title<'a> {
    Title::from(format!(
        " [Mode: {}] [Patch: {}] [Region: {}] ",
        ctx.mode, ctx.version, ctx.region
    ))
    .position(Position::Bottom)
    .alignment(Alignment::Right)
}

pub fn make<'a>(ctx: &'a AppContext) -> impl Widget + 'a {
    let app_border = Block::default()
        .title(
            Title::from(format!(" uggo v{} ", env!("CARGO_PKG_VERSION")))
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from(" [Help: ?] ")
                .position(Position::Bottom)
                .alignment(Alignment::Left),
        )
        .title(make_bottom_right_title(ctx))
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
        .magenta();

    app_border
}
