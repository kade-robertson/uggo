#[macro_use]
extern crate prettytable;

use colored::*;
use ctrlc;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use league_client_connector::LeagueClientConnector;

use prettytable::{format, Table};
use std::io;
use std::io::Write;
use std::process::exit;
use strum::IntoEnumIterator;
use text_io::read;

use crate::types::client_runepage::NewRunePage;
use crate::types::client_summoner::ClientSummoner;

mod api;
mod client_api;
mod config;
mod mappings;
mod styling;
mod util;
mod types {
    pub mod champion;
    pub mod client_runepage;
    pub mod client_summoner;
    pub mod item;
    pub mod matchups;
    pub mod overview;
    pub mod rune;
    pub mod summonerspell;
}

enum ExitReasons {
    Neutral = 0,
    CouldNotGetVersion,
    CouldNotGetChampData,
    CouldNotGetItemData,
    CouldNotGetRuneData,
    CouldNotGetSpellData,
    CouldNotGetUggAPIVersions,
}

static DEFAULT_ROLE: mappings::Role = mappings::Role::Automatic;
static DEFAULT_REGION: mappings::Region = mappings::Region::World;

fn main() {
    ctrlc::set_handler(move || {
        println!("\nExiting...");
        exit(ExitReasons::Neutral as i32);
    })
    .expect("Couldn't handle Ctrl+C");

    let version = match api::get_current_version() {
        Some(data) => data,
        None => {
            util::log_error("Could not get current patch version, exiting...");
            exit(ExitReasons::CouldNotGetVersion as i32);
        }
    };
    util::log_info(&format!(
        "Getting data for patch {}...",
        version.green().bold()
    ));

    let champ_data = match api::get_champ_data(&version) {
        Some(data) => data,
        None => {
            util::log_error("Could not download champ data, exiting...");
            exit(ExitReasons::CouldNotGetChampData as i32);
        }
    };
    #[cfg(debug_assertions)]
    util::log_info(&format!(
        "- Got data for {} champ(s).",
        champ_data.keys().len().to_string().green().bold()
    ));

    let item_data = match api::get_items(&version) {
        Some(data) => data,
        None => {
            util::log_error("Could not download item data, exiting...");
            exit(ExitReasons::CouldNotGetItemData as i32);
        }
    };
    #[cfg(debug_assertions)]
    util::log_info(&format!(
        "- Got data for {} items(s).",
        item_data.keys().len().to_string().green().bold()
    ));

    let rune_data = match api::get_runes(&version) {
        Some(data) => data,
        None => {
            util::log_error("Could not download rune data, exiting...");
            exit(ExitReasons::CouldNotGetRuneData as i32);
        }
    };
    #[cfg(debug_assertions)]
    util::log_info(&format!(
        "- Got data for {} rune(s).",
        rune_data.keys().len().to_string().green().bold()
    ));

    let spell_data = match api::get_summoner_spells(&version) {
        Some(data) => data,
        None => {
            util::log_error("Could not download summoner spell data, exiting...");
            exit(ExitReasons::CouldNotGetSpellData as i32);
        }
    };
    #[cfg(debug_assertions)]
    util::log_info(&format!(
        "- Got data for {} summoner spell(s).",
        spell_data.keys().len().to_string().green().bold()
    ));

    let mut patch_version_split = version.split(".").collect::<Vec<&str>>();
    patch_version_split.remove(patch_version_split.len() - 1);
    let patch_version = patch_version_split.join("_");

    let ugg_api_versions = match api::get_ugg_api_versions(&patch_version) {
        Some(data) => data,
        None => {
            util::log_error("Could not download u.gg api version data, exiting...");
            exit(ExitReasons::CouldNotGetUggAPIVersions as i32);
        }
    };
    #[cfg(debug_assertions)]
    util::log_info(&format!(
        "- Got u.gg API versions for {} patch(es).",
        spell_data.keys().len().to_string().green().bold()
    ));

    let mut mode = mappings::Mode::Normal;

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let client_lockfile = match LeagueClientConnector::parse_lockfile() {
        Ok(lockfile) => Some(lockfile),
        Err(_) => None,
    };

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    let client_lockfile = None;

    #[cfg(all(debug_assertions, any(target_os = "windows", target_os = "macos")))]
    if !client_lockfile.as_ref().is_none() {
        let lockfile = client_lockfile.clone().unwrap();
        util::log_info(&format!(
            "- Found client running at https://127.0.0.1:{}/",
            lockfile.port
        ));
    }

    let mut client_summoner: Option<Box<ClientSummoner>> = None;
    if !client_lockfile.as_ref().is_none() {
        client_summoner = match client_api::get_summoner_info(&client_lockfile.clone().unwrap()) {
            Some(data) => Some(data),
            None => None,
        }
    }

    #[cfg(debug_assertions)]
    if !client_summoner.as_ref().is_none() {
        let summoner = client_summoner.unwrap();
        util::log_info(&format!(
            "- Found summoner {} (id: {})",
            summoner.display_name, summoner.summoner_id
        ));
    }

    loop {
        print!("query> ");
        io::stdout().flush().unwrap();
        let user_input: String = read!("{}\n");
        let clean_user_input = user_input.trim();
        let user_input_split = clean_user_input
            .split(',')
            .map(|s: &str| s.trim())
            .collect::<Vec<&str>>();

        if clean_user_input == "modes" {
            util::log_info("Available modes:");
            mappings::Mode::iter().for_each(|m| util::log_info(format!("- {:?}", m).as_str()));
            continue;
        }

        if clean_user_input.starts_with("mode") {
            let mode_to_set = clean_user_input.split(' ').collect::<Vec<&str>>();
            if mode_to_set.len() > 1 {
                mode = mappings::get_mode(mode_to_set[1]);
                util::log_info(format!("Switching mode to {:?}...", mode).as_str());
                continue;
            } else {
                util::log_info(format!("Current mode: {:?}", mode).as_str());
                continue;
            }
        }

        let mut query_champ_name = "";
        let mut query_role = DEFAULT_ROLE;
        let mut query_region = DEFAULT_REGION;

        if user_input_split.len() < 1 || user_input_split.len() > 3 || user_input_split[0] == "" {
            util::log_info("This doesn't look like a valid query.");
            util::log_info("Query format is <champion>[,<role>][,<region>]");
            continue;
        }
        if user_input_split.len() >= 1 {
            query_champ_name = user_input_split[0];
        }
        if user_input_split.len() >= 2 {
            let try_role = mappings::get_role(&user_input_split[1]);
            if try_role == query_role {
                query_region = mappings::get_region(&user_input_split[1]);
            } else {
                query_role = try_role;
            }
        }
        if user_input_split.len() == 3 {
            query_role = mappings::get_role(&user_input_split[1]);
            query_region = mappings::get_region(&user_input_split[2]);
        }

        let query_champ = util::find_champ(query_champ_name, &champ_data);

        let formatted_champ_name = query_champ.name.as_str().green().bold();

        let mut query_message = vec![format!("Looking up info for {}", formatted_champ_name)];
        if query_role != DEFAULT_ROLE {
            query_message.push(format!(
                ", playing {}",
                query_role.to_string().blue().bold()
            ));
        }
        if query_region != DEFAULT_REGION {
            query_message.push(format!(", in {}", query_region.to_string().red().bold()));
        }
        query_message.push("...".to_string());
        util::log_info(query_message.concat().as_str());

        let (overview_role, champ_overview) = match api::get_stats(
            &patch_version.as_str(),
            query_champ,
            query_role,
            query_region,
            mode,
            &ugg_api_versions,
        ) {
            Some(data) => *data,
            None => {
                util::log_error(
                    format!("Couldn't get required data for {}.", formatted_champ_name).as_str(),
                );
                continue;
            }
        };

        let matchups = api::get_matchups(
            &patch_version.as_str(),
            query_champ,
            overview_role,
            query_region,
            mode,
            &ugg_api_versions,
        );

        let mut stats_message = vec![format!("Build for {}", formatted_champ_name)];
        let mut true_length = 10 /* "Build for " */ + query_champ.name.len();
        if overview_role != mappings::Role::None {
            stats_message.push(format!(
                ", playing {} lane",
                overview_role.to_string().blue().bold()
            ));
            true_length += 15 /* ", playing  lane" */ + overview_role.to_string().len();
        }
        let stats_message_str = stats_message.concat();
        println!(" {}", "-".repeat(true_length));
        println!(" {}", stats_message_str);
        println!(" {}", "-".repeat(true_length));

        let champ_runes = util::group_runes(&champ_overview.runes.rune_ids, &rune_data);
        let mut rune_table = Table::new();
        rune_table.set_format(*format::consts::FORMAT_CLEAN);
        rune_table.add_row(row![
            styling::format_rune_group(&champ_runes[0].0.as_str()),
            "",
            styling::format_rune_group(&champ_runes[1].0.as_str()),
            ""
        ]);
        rune_table.add_row(row![
            &champ_runes[0].1[0].rune.name,
            styling::format_rune_position(&champ_runes[0].1[0]),
            format!(
                "{} (Row {})",
                &champ_runes[1].1[0].rune.name, &champ_runes[1].1[0].slot
            ),
            styling::format_rune_position(&champ_runes[1].1[0])
        ]);
        rune_table.add_row(row![
            &champ_runes[0].1[1].rune.name,
            styling::format_rune_position(&champ_runes[0].1[1]),
            format!(
                "{} (Row {})",
                &champ_runes[1].1[1].rune.name, &champ_runes[1].1[1].slot
            ),
            styling::format_rune_position(&champ_runes[1].1[1])
        ]);
        rune_table.add_row(row![
            &champ_runes[0].1[2].rune.name,
            styling::format_rune_position(&champ_runes[0].1[2])
        ]);
        rune_table.add_row(row![
            &champ_runes[0].1[3].rune.name,
            styling::format_rune_position(&champ_runes[0].1[3])
        ]);
        rune_table.printstd();

        println!();
        println!(" {}", "Shards:".magenta().bold());
        util::process_shards(&champ_overview.shards.shard_ids)
            .iter()
            .for_each(|shard| println!(" {}", shard));

        println!();
        println!(
            " {} {}",
            "Spells:".yellow().bold(),
            format!(
                "{}, {}",
                &spell_data[&champ_overview.summoner_spells.spell_ids[0]],
                &spell_data[&champ_overview.summoner_spells.spell_ids[1]]
            )
        );

        println!();
        println!(
            " {} {}",
            "Ability Order:".bright_cyan().bold(),
            champ_overview
                .abilities
                .ability_max_order
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" -> ")
        );

        let mut item_table = Table::new();
        item_table.set_format(*format::consts::FORMAT_CLEAN);
        item_table.add_row(row![
            r->"Starting:".green(),
            util::process_items(&champ_overview.starting_items.item_ids, &item_data)
        ]);
        item_table.add_row(row![
            r->"Core:".green(),
            util::process_items(&champ_overview.core_items.item_ids, &item_data)
        ]);
        item_table.add_row(row![
            r->"4th:".green(),
            util::process_items(&champ_overview.item_4_options.iter().map(|x| x.id).collect::<Vec<i64>>(), &item_data)
        ]);
        item_table.add_row(row![
            r->"5th:".green(),
            util::process_items(&champ_overview.item_5_options.iter().map(|x| x.id).collect::<Vec<i64>>(), &item_data)
        ]);
        item_table.add_row(row![
            r->"6th:".green(),
            util::process_items(&champ_overview.item_6_options.iter().map(|x| x.id).collect::<Vec<i64>>(), &item_data)
        ]);
        println!();
        item_table.printstd();

        if !matchups.is_none() {
            let safe_matchups = *matchups.clone().unwrap();
            let mut matchup_table = Table::new();
            matchup_table.set_format(*format::consts::FORMAT_CLEAN);
            matchup_table.add_row(row![
                r->"Best Matchups:".cyan().bold(),
                safe_matchups
                    .best_matchups
                    .into_iter()
                    .map(|m| util::find_champ_by_key(m.champion_id, &champ_data)
                        .unwrap()
                        .name
                        .as_str())
                    .collect::<Vec<&str>>()
                    .join(", ")
            ]);
            matchup_table.add_row(row![
                r->"Worst Matchups:".red().bold(),
                safe_matchups
                    .worst_matchups
                    .into_iter()
                    .map(|m| util::find_champ_by_key(m.champion_id, &champ_data)
                        .unwrap()
                        .name
                        .as_str())
                    .collect::<Vec<&str>>()
                    .join(", ")
            ]);
            println!();
            matchup_table.printstd();
        }

        if champ_overview.low_sample_size {
            println!();
            println!(
                " {} Data has a low sample size for this combination!",
                "Warning:".yellow().bold()
            );
        }

        if !client_lockfile.as_ref().is_none() {
            match client_api::get_current_rune_page(&client_lockfile.clone().unwrap()) {
                Some(data) => {
                    let (primary_style_id, sub_style_id, selected_perk_ids) =
                        util::generate_perk_array(&champ_runes, &champ_overview.shards.shard_ids);
                    client_api::update_rune_page(
                        &client_lockfile.clone().unwrap(),
                        &data.id,
                        &NewRunePage {
                            name: if mode == mappings::Mode::ARAM {
                                format!("uggo: {}, ARAM", &query_champ.name)
                            } else {
                                format!("uggo: {}, {}", &query_champ.name, &overview_role)
                            },
                            primary_style_id,
                            sub_style_id,
                            selected_perk_ids,
                        },
                    );
                }
                _ => (),
            }
        }

        println!();
    }
}
