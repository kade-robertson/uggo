use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Widget},
};

use crate::context::AppContext;

#[allow(clippy::cast_precision_loss)]
#[cfg(debug_assertions)]
fn make_bottom_right_title<'a>(ctx: &'a AppContext) -> Line<'a> {
    Line::from(format!(
        " [Mode: {}] [Patch: {}] [Region: {}] [Render: {:.2}ms] ",
        ctx.mode,
        ctx.version,
        ctx.region,
        ctx.last_render_duration
            .map_or(0.0, |d| d.as_micros() as f64 / 1000.0)
    ))
    .right_aligned()
}

#[allow(clippy::cast_precision_loss)]
#[cfg(not(debug_assertions))]
fn make_bottom_right_title<'a>(ctx: &'a AppContext) -> Line<'a> {
    Line::from(format!(
        " [Mode: {}] [Patch: {}] [Region: {}] ",
        ctx.mode, ctx.version, ctx.region
    ))
    .right_aligned()
}

pub fn make<'a>(ctx: &'a AppContext) -> impl Widget + 'a {
    Block::default()
        .title_top(Line::from(format!(" uggo v{} ", env!("CARGO_PKG_VERSION"))).centered())
        .title_bottom(Line::from(" [Help: ?] ").left_aligned())
        .title_bottom(make_bottom_right_title(ctx))
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
        .magenta()
}
