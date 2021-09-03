use colored::*;

use crate::types::rune::RuneExtended;

pub fn format_rune_group(name: &str) -> ColoredString {
    return match name {
        "Precision" => name.yellow().bold(),
        "Domination" => name.red().bold(),
        "Sorcery" => name.purple().bold(),
        "Resolve" => name.green().bold(),
        "Inspiration" => name.blue().bold(),
        _ => "".bold(),
    };
}

pub fn format_rune_position(rune: &RuneExtended) -> String {
    let mut position_message = String::new();
    position_message.push('[');
    let mut index = 0;
    while index < rune.siblings {
        if index == rune.index {
            position_message.push_str("·".red().bold().to_string().as_str());
        } else {
            position_message.push('·');
        }
        index += 1;
    }
    position_message.push(']');
    return position_message;
}
