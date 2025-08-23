use ddragon::models::runes::RuneElement;
use std::collections::HashMap;

use ugg_types::rune::RuneExtended;

pub fn group_runes<'a>(
    rune_ids: &Vec<i64>,
    rune_data: &'a HashMap<i64, RuneExtended<RuneElement>>,
) -> [(String, Vec<&'a RuneExtended<RuneElement>>); 2] {
    let mut grouped_runes: Vec<(String, Vec<&'a RuneExtended<RuneElement>>)> = Vec::new();

    for rune in rune_ids {
        let rune_info = &rune_data[rune];
        match grouped_runes.iter().position(|r| r.0 == rune_info.parent) {
            Some(group_index) => grouped_runes[group_index].1.push(rune_info),
            None => grouped_runes.push((rune_info.parent.to_string(), vec![rune_info])),
        }
    }

    // Make sure primary rune is first
    if grouped_runes[0].1.len() != 4 {
        grouped_runes.reverse();
    }

    for group in &mut grouped_runes {
        group.1.sort_by(|&a, &b| a.slot.cmp(&b.slot));
    }

    [grouped_runes[0].to_owned(), grouped_runes[1].to_owned()]
}

pub fn generate_perk_array(
    runes: &[(String, Vec<&RuneExtended<RuneElement>>)],
    shards: &[i64],
) -> (i64, i64, Vec<i64>) {
    let mut perk_list: Vec<i64> = Vec::new();
    perk_list.append(&mut runes[0].1.iter().map(|el| el.rune.id).collect::<Vec<i64>>());
    perk_list.append(&mut runes[1].1.iter().map(|el| el.rune.id).collect::<Vec<i64>>());
    perk_list.append(&mut shards.to_vec());
    (runes[0].1[0].parent_id, runes[1].1[0].parent_id, perk_list)
}
