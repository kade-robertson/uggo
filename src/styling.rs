use colored::*;

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
