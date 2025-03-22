use std::collections::HashMap;

use ddragon::models::items::Item;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, Widget},
};
use ugg_types::{
    arena_overview::{ArenaOverviewData, PrismaticItem},
    default_overview::{LateItem, OverviewData},
};

fn make_item_block<'a>(title: &str) -> Block<'a> {
    Block::default()
        .white()
        .title(format!(" {title} "))
        .title_style(Style::default().bold())
        .borders(Borders::ALL)
}

pub fn make_placeholder() -> impl Widget {
    make_item_block("Items")
}

fn make_list_from_lateitems<'a>(
    name: &str,
    late_items: &[LateItem],
    items: &HashMap<String, Item>,
) -> List<'a> {
    List::new(
        late_items
            .iter()
            .filter_map(|i| {
                items
                    .get(&i.id.to_string())
                    .map(|it| ListItem::new(it.name.clone()))
            })
            .collect::<Vec<_>>(),
    )
    .block(make_item_block(name))
}

fn make_list_from_item_ids<'a>(
    name: &str,
    item_ids: &[i64],
    items: &HashMap<String, Item>,
) -> List<'a> {
    List::new(
        item_ids
            .iter()
            .filter_map(|i| {
                items
                    .get(&i.to_string())
                    .map(|it| ListItem::new(it.name.clone()))
            })
            .collect::<Vec<_>>(),
    )
    .block(make_item_block(name))
}

fn make_list_from_prismatic_items<'a>(
    name: &str,
    late_items: &[PrismaticItem],
    items: &HashMap<String, Item>,
) -> List<'a> {
    List::new(
        late_items
            .iter()
            .filter_map(|i| {
                items
                    .get(&i.id.to_string())
                    .map(|it| ListItem::new(it.name.clone()))
            })
            .collect::<Vec<_>>(),
    )
    .block(make_item_block(name))
}

pub fn make_default(overview: &OverviewData, items: &HashMap<String, Item>) -> [impl Widget; 5] {
    [
        make_list_from_item_ids("Starting Items", &overview.starting_items.item_ids, items),
        make_list_from_item_ids("Core Items", &overview.core_items.item_ids, items),
        make_list_from_lateitems("4th Items", &overview.item_4_options, items),
        make_list_from_lateitems("5th Items", &overview.item_5_options, items),
        make_list_from_lateitems("6th Items", &overview.item_6_options, items),
    ]
}

pub fn make_arena(overview: &ArenaOverviewData, items: &HashMap<String, Item>) -> [impl Widget; 7] {
    [
        make_list_from_item_ids("Starting Items", &overview.starting_items.item_ids, items),
        make_list_from_item_ids("2nd & 3rd Items", &overview.core_items.item_ids, items),
        make_list_from_lateitems("4th Items", &overview.item_4_options, items),
        make_list_from_lateitems("5th Items", &overview.item_5_options, items),
        make_list_from_lateitems("6th Items", &overview.item_6_options, items),
        make_list_from_lateitems("Consumables", &overview.consumables, items),
        make_list_from_prismatic_items("Prismatic Items", &overview.prismatic_items, items),
    ]
}
