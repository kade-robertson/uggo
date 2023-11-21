use ddragon::models::runes::RuneElement;
use ugg_types::rune::RuneExtended;

pub fn format_rune_position(rune: &RuneExtended<RuneElement>) -> String {
    let mut position_message = String::new();
    position_message.push('[');
    let mut index = 0;
    while index < rune.siblings {
        if index == rune.index {
            position_message.push('●');
        } else {
            position_message.push('·');
        }
        index += 1;
    }
    position_message.push(']');
    position_message
}

pub fn format_ability_level_order(ability_order: &[char], ability: char) -> String {
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
