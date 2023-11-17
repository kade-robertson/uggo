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
