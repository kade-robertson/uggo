use colored::*;

use crate::types::rune::RuneExtended;

pub fn format_rune_text(group: &str, text: Option<&str>) -> ColoredString {
    let output_text = match text {
        Some(val) => val,
        None => group,
    };
    return match group {
        "Precision" => output_text.yellow().bold(),
        "Domination" => output_text.red().bold(),
        "Sorcery" => output_text.purple().bold(),
        "Resolve" => output_text.green().bold(),
        "Inspiration" => output_text.blue().bold(),
        _ => "".bold(),
    };
}

pub fn format_rune_group(name: &str) -> ColoredString {
    return format_rune_text(name, None);
}

pub fn format_rune_position(rune: &RuneExtended) -> String {
    let mut position_message = String::new();
    position_message.push('[');
    let mut index = 0;
    while index < rune.siblings {
        if index == rune.index {
            position_message.push_str(
                format_rune_text(rune.parent.as_str(), Some("·"))
                    .to_string()
                    .as_str(),
            );
        } else {
            position_message.push('·');
        }
        index += 1;
    }
    position_message.push(']');
    return position_message;
}
