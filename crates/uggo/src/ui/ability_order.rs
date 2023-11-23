use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};
use ugg_types::overview::OverviewData;

fn format_ability_level_order(ability_order: &[char], ability: char) -> String {
    ability_order
        .iter()
        .copied()
        .map(|c| {
            if c == ability {
                "●".to_string()
            } else {
                " ".to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

const ABILITY_LEFT_OFFSET: u16 = 4;
const ABILITY_WIDTH: u16 = (5 /* Q */ + 5 /* W */ + 5 /* E */ + 3 /* R */) * 2 /* spaces */;

pub fn make_ability_order_placeholder() -> impl Widget {
    Block::default()
        .white()
        .title(" Ability Order ")
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
}

pub fn make_ability_order(bounds: Rect, overview: &OverviewData) -> Vec<(impl Widget, Rect)> {
    vec![
        // Draw Q |
        (
            Paragraph::new("Q│ "),
            Rect::new(bounds.left(), bounds.top(), ABILITY_LEFT_OFFSET, 1),
        ),
        // Draw Q abilities
        (
            Paragraph::new(format_ability_level_order(
                &overview.abilities.ability_order,
                'Q',
            ))
            .style(Style::default().fg(Color::Cyan).bold()),
            Rect::new(bounds.left() + 3, bounds.top(), ABILITY_WIDTH, 1),
        ),
        // Draw W |
        (
            Paragraph::new("W│ "),
            Rect::new(bounds.left(), bounds.top() + 1, ABILITY_LEFT_OFFSET, 1),
        ),
        // Draw W abilities
        (
            Paragraph::new(format_ability_level_order(
                &overview.abilities.ability_order,
                'W',
            ))
            .style(Style::default().fg(Color::Yellow).bold()),
            Rect::new(bounds.left() + 3, bounds.top() + 1, ABILITY_WIDTH, 1),
        ),
        // Draw E |
        (
            Paragraph::new("E│ "),
            Rect::new(bounds.left(), bounds.top() + 2, ABILITY_LEFT_OFFSET, 1),
        ),
        // Draw E abilities
        (
            Paragraph::new(format_ability_level_order(
                &overview.abilities.ability_order,
                'E',
            ))
            .style(Style::default().fg(Color::Green).bold()),
            Rect::new(bounds.left() + 3, bounds.top() + 2, ABILITY_WIDTH, 1),
        ),
        // Draw R |
        (
            Paragraph::new("R│ "),
            Rect::new(bounds.left(), bounds.top() + 3, ABILITY_LEFT_OFFSET, 1),
        ),
        // Draw R abilities
        (
            Paragraph::new(format_ability_level_order(
                &overview.abilities.ability_order,
                'R',
            ))
            .style(Style::default().fg(Color::Red).bold()),
            Rect::new(bounds.left() + 3, bounds.top() + 3, ABILITY_WIDTH, 1),
        ),
    ]
}
