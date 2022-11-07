#[macro_use]
extern crate prettytable;

use colored::*;

#[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
use league_client_connector::LeagueClientConnector;

use anyhow::Result;
use clap::Parser;
use prettytable::{format, Table};
use std::io;
use std::io::Write;
use std::process::exit;
use strum::IntoEnumIterator;
use text_io::read;

use crate::styling::format_ability_order;

#[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
use crate::types::client_runepage::NewRunePage;

mod api;
#[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
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

static DEFAULT_MODE: mappings::Mode = mappings::Mode::Normal;
static DEFAULT_ROLE: mappings::Role = mappings::Role::Automatic;
static DEFAULT_REGION: mappings::Region = mappings::Region::World;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the champion you want to match. A best effort will be made
    /// to find the champ if it's only a partial query.
    ///
    /// If left blank, will open the interactive version of uggo.
    champ: Option<String>,

    #[arg(short, long, default_value_t = DEFAULT_MODE)]
    mode: mappings::Mode,

    #[arg(short, long, default_value_t = DEFAULT_ROLE)]
    role: mappings::Role,

    #[arg(short = 'R', long, default_value_t = DEFAULT_REGION)]
    region: mappings::Region,
}

fn fetch(
    ugg: &api::UggApi,
    #[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
    client: Option<&client_api::ClientAPI>,
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_feature = "clippy")))] _client: Option<bool>,
    champ: &str,
    mode: mappings::Mode,
    role: mappings::Role,
    region: mappings::Region,
) {
    let query_champ = ugg.find_champ(champ);

    let formatted_champ_name = query_champ.name.as_str().green().bold();

    let mut query_message = vec![format!("Looking up info for {}", formatted_champ_name)];
    if role != DEFAULT_ROLE {
        query_message.push(format!(", playing {}", role.to_string().blue().bold()));
    }
    if region != DEFAULT_REGION {
        query_message.push(format!(", in {}", region.to_string().red().bold()));
    }
    query_message.push("...".to_string());
    util::log_info(query_message.concat().as_str());

    let (overview_role, champ_overview) = match ugg.get_stats(query_champ, role, region, mode) {
        Ok(data) => *data,
        Err(_) => {
            util::log_error(
                format!("Couldn't get required data for {}.", formatted_champ_name).as_str(),
            );
            return;
        }
    };

    let matchups = ugg.get_matchups(query_champ, overview_role, region, mode);

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

    let champ_runes = util::group_runes(&champ_overview.runes.rune_ids, &ugg.runes);
    let mut rune_table = Table::new();
    rune_table.set_format(*format::consts::FORMAT_CLEAN);
    rune_table.add_row(row![
        styling::format_rune_group(champ_runes[0].0.as_str()),
        "",
        styling::format_rune_group(champ_runes[1].0.as_str()),
        ""
    ]);
    rune_table.add_row(row![
        &champ_runes[0].1[0].rune.name,
        styling::format_rune_position(champ_runes[0].1[0]),
        format!(
            "{} (Row {})",
            &champ_runes[1].1[0].rune.name, &champ_runes[1].1[0].slot
        ),
        styling::format_rune_position(champ_runes[1].1[0])
    ]);
    rune_table.add_row(row![
        &champ_runes[0].1[1].rune.name,
        styling::format_rune_position(champ_runes[0].1[1]),
        format!(
            "{} (Row {})",
            &champ_runes[1].1[1].rune.name, &champ_runes[1].1[1].slot
        ),
        styling::format_rune_position(champ_runes[1].1[1])
    ]);
    rune_table.add_row(row![
        &champ_runes[0].1[2].rune.name,
        styling::format_rune_position(champ_runes[0].1[2])
    ]);
    rune_table.add_row(row![
        &champ_runes[0].1[3].rune.name,
        styling::format_rune_position(champ_runes[0].1[3])
    ]);
    rune_table.printstd();

    println!();
    println!(" {}", "Shards:".magenta().bold());
    util::process_shards(&champ_overview.shards.shard_ids)
        .iter()
        .for_each(|shard| println!(" {}", shard));

    println!();
    println!(
        " {} {}, {}",
        "Spells:".yellow().bold(),
        &ugg.summoner_spells[&champ_overview.summoner_spells.spell_ids[0]],
        &ugg.summoner_spells[&champ_overview.summoner_spells.spell_ids[1]]
    );

    println!();
    println!(" {}", "Ability Order:".bright_cyan().bold(),);
    format_ability_order(&champ_overview.abilities.ability_order).printstd();

    let mut item_table = Table::new();
    item_table.set_format(*format::consts::FORMAT_CLEAN);
    item_table.add_row(row![
        r->"Starting:".green(),
        util::process_items(&champ_overview.starting_items.item_ids, &ugg.items)
    ]);
    item_table.add_row(row![
        r->"Core:".green(),
        util::process_items(&champ_overview.core_items.item_ids, &ugg.items)
    ]);
    item_table.add_row(row![
        r->"4th:".green(),
        util::process_items(&champ_overview.item_4_options.iter().map(|x| x.id).collect::<Vec<i64>>(), &ugg.items)
    ]);
    item_table.add_row(row![
        r->"5th:".green(),
        util::process_items(&champ_overview.item_5_options.iter().map(|x| x.id).collect::<Vec<i64>>(), &ugg.items)
    ]);
    item_table.add_row(row![
        r->"6th:".green(),
        util::process_items(&champ_overview.item_6_options.iter().map(|x| x.id).collect::<Vec<i64>>(), &ugg.items)
    ]);
    println!();
    item_table.printstd();

    if let Ok(safe_matchups) = matchups {
        let mut matchup_table = Table::new();
        matchup_table.set_format(*format::consts::FORMAT_CLEAN);
        matchup_table.add_row(row![
            r->"Best Matchups:".cyan().bold(),
            safe_matchups
                .best_matchups
                .into_iter()
                .map(|m| util::find_champ_by_key(m.champion_id, &ugg.champ_data)
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
                .map(|m| util::find_champ_by_key(m.champion_id, &ugg.champ_data)
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

    #[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
    match client {
        Some(ref api) => match api.get_current_rune_page() {
            Some(data) => {
                let (primary_style_id, sub_style_id, selected_perk_ids) =
                    util::generate_perk_array(&champ_runes, &champ_overview.shards.shard_ids);
                api.update_rune_page(
                    &data.id,
                    &NewRunePage {
                        name: match mode {
                            mappings::Mode::ARAM => {
                                format!("uggo: {}, ARAM", &query_champ.name)
                            }
                            mappings::Mode::URF => format!("uggo: {}, URF", &query_champ.name),
                            _ => format!("uggo: {}, {}", &query_champ.name, &overview_role),
                        },
                        primary_style_id,
                        sub_style_id,
                        selected_perk_ids,
                    },
                );
            }
            _ => (),
        },
        None => (),
    }
}

fn main() -> Result<()> {
    ctrlc::set_handler(move || {
        println!("\nExiting...");
        exit(0);
    })?;

    let ugg = api::UggApi::new()?;
    let mut mode = mappings::Mode::Normal;

    #[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
    let client_lockfile = LeagueClientConnector::parse_lockfile().ok();

    #[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
    let mut clientapi: Option<client_api::ClientAPI> = None;

    #[cfg(all(
        debug_assertions,
        any(target_os = "windows", target_os = "macos", target_feature = "clippy")
    ))]
    if !client_lockfile.as_ref().is_none() {
        let lockfile = client_lockfile.clone().unwrap();
        util::log_info(&format!(
            "- Found client running at https://127.0.0.1:{}/",
            lockfile.port
        ));
        clientapi = Some(client_api::ClientAPI::new(lockfile));
    }

    #[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
    if !client_lockfile.as_ref().is_none() {
        clientapi = Some(client_api::ClientAPI::new(client_lockfile.clone().unwrap()));
    }

    #[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
    match clientapi {
        Some(ref api) => match api.get_summoner_info() {
            Some(summoner) => {
                #[cfg(debug_assertions)]
                util::log_info(&format!(
                    "- Found summoner {} (id: {})",
                    summoner.display_name, summoner.summoner_id
                ));
            }
            None => (),
        },
        None => (),
    }

    let parsed_args = Args::parse();

    if let Some(champ_name) = parsed_args.champ {
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_feature = "clippy")))]
        fetch(
            &ugg,
            None,
            &champ_name,
            parsed_args.mode,
            parsed_args.role,
            parsed_args.region,
        );

        #[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
        fetch(
            &ugg,
            &clientapi,
            &champ_name,
            parsed_args.mode,
            parsed_args.role,
            parsed_args.region,
        );
        return Ok(());
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
                mode = mappings::Mode::from(mode_to_set[1]);
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

        if user_input_split.is_empty()
            || user_input_split.len() > 3
            || user_input_split[0].is_empty()
        {
            util::log_info("This doesn't look like a valid query.");
            util::log_info("Query format is <champion>[,<role>][,<region>]");
            continue;
        }
        if !user_input_split.is_empty() {
            query_champ_name = user_input_split[0];
        }
        if user_input_split.len() >= 2 {
            let try_role = mappings::get_role(user_input_split[1]);
            if try_role == query_role {
                query_region = mappings::get_region(user_input_split[1]);
            } else {
                query_role = try_role;
            }
        }
        if user_input_split.len() == 3 {
            query_role = mappings::get_role(user_input_split[1]);
            query_region = mappings::get_region(user_input_split[2]);
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_feature = "clippy")))]
        fetch(&ugg, None, query_champ_name, mode, query_role, query_region);

        #[cfg(any(target_os = "windows", target_os = "macos", target_feature = "clippy"))]
        fetch(
            &ugg,
            clientapi,
            query_champ_name,
            mode,
            query_role,
            query_region,
        );

        println!();
    }
}
