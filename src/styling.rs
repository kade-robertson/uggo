use colored::{ColoredString, Colorize};
use prettytable::{format, Table};

use crate::types::rune::RuneExtended;

pub fn format_rune_text(group: &str, text: Option<&str>) -> ColoredString {
    let output_text = match text {
        Some(val) => val,
        None => group,
    };
    match group {
        "Precision" => output_text.yellow().bold(),
        "Domination" => output_text.red().bold(),
        "Sorcery" => output_text.purple().bold(),
        "Resolve" => output_text.green().bold(),
        "Inspiration" => output_text.blue().bold(),
        _ => "".bold(),
    }
}

pub fn format_rune_group(name: &str) -> ColoredString {
    format_rune_text(name, None)
}

pub fn format_rune_position(rune: &RuneExtended) -> String {
    let mut position_message = String::new();
    position_message.push('[');
    let mut index = 0;
    while index < rune.siblings {
        if index == rune.index {
            position_message.push_str(
                format_rune_text(rune.parent.as_str(), Some("●"))
                    .to_string()
                    .as_str(),
            );
        } else {
            position_message.push_str("·".black().bold().to_string().as_str());
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

pub fn format_ability_order(ability_order: &[char]) -> Table {
    let mut ability_table = Table::new();
    ability_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    ability_table.add_row(row![
        "Q",
        format_ability_level_order(ability_order, 'Q').cyan().bold()
    ]);
    ability_table.add_row(row![
        "W",
        format_ability_level_order(ability_order, 'W')
            .yellow()
            .bold()
    ]);
    ability_table.add_row(row![
        "E",
        format_ability_level_order(ability_order, 'E')
            .green()
            .bold()
    ]);
    ability_table.add_row(row![
        "R",
        format_ability_level_order(ability_order, 'R').red().bold()
    ]);
    ability_table
}
