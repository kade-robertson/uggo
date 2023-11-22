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
pub fn make_app_border(ctx: &AppContext) -> impl Widget {
    let app_border = Block::default()
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
            Title::from(format!(
                " [Mode: {}] [Render: {:.2}ms] ",
                ctx.mode,
                ctx.last_render_duration
                    .map_or(0.0, |d| d.as_micros() as f64 / 1000.0)
            ))
            .position(Position::Bottom)
            .alignment(Alignment::Right),
        )
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
        .magenta();

    app_border
}
