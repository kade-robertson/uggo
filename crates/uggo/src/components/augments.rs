use std::collections::HashMap;

use ddragon::models::cdragon::AugmentRarity;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, Widget},
};
use ugg_types::arena_overview::{ArenaOverviewData, Augment};

fn make_augment_block<'a>(title: &str) -> Block<'a> {
    Block::default()
        .white()
        .title(format!(" {title} "))
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
}

pub fn make_placeholder() -> impl Widget {
    make_augment_block("Augments")
}

fn make_list_from_augments<'a>(
    name: &str,
    augments: &[Augment],
    game_augments: &HashMap<i64, ddragon::models::Augment>,
    rarity: &AugmentRarity,
) -> List<'a> {
    List::new(
        augments
            .iter()
            .filter_map(|i| {
                game_augments
                    .get(&i.id)
                    .filter(|a| a.rarity == *rarity)
                    .map(|it| ListItem::new(it.name.clone()))
            })
            .take(6)
            .collect::<Vec<_>>(),
    )
    .block(make_augment_block(name))
}

pub fn make(
    overview: &ArenaOverviewData,
    game_augments: &HashMap<i64, ddragon::models::Augment>,
) -> [impl Widget; 3] {
    [
        make_list_from_augments(
            "Silver Augments",
            &overview.augments,
            game_augments,
            &AugmentRarity::Silver,
        ),
        make_list_from_augments(
            "Gold Augments",
            &overview.augments,
            game_augments,
            &AugmentRarity::Gold,
        ),
        make_list_from_augments(
            "Prismatic Augments",
            &overview.augments,
            game_augments,
            &AugmentRarity::Prismatic,
        ),
    ]
}
