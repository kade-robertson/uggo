#[macro_use]
extern crate prettytable;

#[cfg(debug_assertions)]
use chrono;
use colored::*;
use ctrlc;
use prettytable::{format, Table};
use std::io;
use std::io::Write;
use std::process::exit;
use std::str::FromStr;
use text_io::read;

mod api;
mod mappings;
mod styling;
mod util;
mod types {
    pub mod champion;
    pub mod item;
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
}

#[cfg(debug_assertions)]
static TIME_FORMAT: &str = "[%Y-%m-%d %H:%M:%S%.3f] ";

static DEFAULT_ROLE: mappings::Role = mappings::Role::Automatic;
static DEFAULT_REGION: mappings::Region = mappings::Region::World;

fn log_error(msg: &str) {
    #[cfg(debug_assertions)]
    {
        let now = chrono::Local::now();
        eprint!("{}", now.format(TIME_FORMAT).to_string().as_str());
    }
    eprint!("{} ", "Error:".red().bold());
    eprintln!("{}", msg);
}

fn log_info(msg: &str) {
    let mut message = String::new();
    #[cfg(debug_assertions)]
    {
        let now = chrono::Local::now();
        message.push_str(now.format(TIME_FORMAT).to_string().as_str());
    }
    message.push_str(msg);
    println!("{}", message);
}

fn main() {
    ctrlc::set_handler(move || {
        println!("\nExiting...");
        exit(ExitReasons::Neutral as i32);
    })
    .expect("Couldn't handle Ctrl+C");

    let version = match api::get_current_version() {
        Some(data) => data,
        None => {
            log_error("Could not get current patch version, exiting...");
            exit(ExitReasons::CouldNotGetVersion as i32);
        }
    };
    log_info(&format!(
        "Getting data for patch {}...",
        version.green().bold()
    ));

    let champ_data = match api::get_champ_data(&version) {
        Some(data) => data,
        None => {
            log_error("Could not download champ data, exiting...");
            exit(ExitReasons::CouldNotGetChampData as i32);
        }
    };
    #[cfg(debug_assertions)]
    log_info(&format!(
        "- Got data for {} champ(s).",
        champ_data.keys().len().to_string().green().bold()
    ));

    let item_data = match api::get_items(&version) {
        Some(data) => data,
        None => {
            log_error("Could not download item data, exiting...");
            exit(ExitReasons::CouldNotGetItemData as i32);
        }
    };
    #[cfg(debug_assertions)]
    log_info(&format!(
        "- Got data for {} items(s).",
        item_data.keys().len().to_string().green().bold()
    ));

    let rune_data = match api::get_runes(&version) {
        Some(data) => data,
        None => {
            log_error("Could not download rune data, exiting...");
            exit(ExitReasons::CouldNotGetRuneData as i32);
        }
    };
    #[cfg(debug_assertions)]
    log_info(&format!(
        "- Got data for {} rune(s).",
        rune_data.keys().len().to_string().green().bold()
    ));

    let spell_data = match api::get_summoner_spells(&version) {
        Some(data) => data,
        None => {
            log_error("Could not download summoner spell data, exiting...");
            exit(ExitReasons::CouldNotGetSpellData as i32);
        }
    };
    #[cfg(debug_assertions)]
    log_info(&format!(
        "- Got data for {} summoner spell(s).",
        spell_data.keys().len().to_string().green().bold()
    ));

    let mut patch_version_split = version.split(".").collect::<Vec<&str>>();
    patch_version_split.remove(patch_version_split.len() - 1);
    let patch_version = patch_version_split.join("_");

    let mut mode = mappings::Mode::Normal;

    loop {
        print!("query> ");
        io::stdout().flush().unwrap();
        let user_input: String = read!("{}\n");
        let user_input_split = user_input.trim().split(',').collect::<Vec<&str>>();

        if user_input.starts_with("mode") {
            let mode_to_set = user_input.trim().split(' ').collect::<Vec<&str>>()[1];
            mode = mappings::Mode::from_str(mode_to_set).unwrap();
            log_info("Switching mode...");
            continue;
        }

        let mut query_champ_name = "";
        let mut query_role = DEFAULT_ROLE;
        let mut query_region = DEFAULT_REGION;

        if user_input_split.len() < 1 || user_input_split.len() > 3 || user_input_split[0] == "" {
            log_info("This doesn't look like a valid query.");
            log_info("Query format is <champion>[,<role>][,<region>]");
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
        log_info(query_message.concat().as_str());

        let (overview_role, champ_overview) = match api::get_stats(
            &patch_version.as_str(),
            query_champ,
            query_role,
            query_region,
            mode,
        ) {
            Some(data) => *data,
            None => {
                log_error(
                    format!("Couldn't get required data for {}.", formatted_champ_name).as_str(),
                );
                continue;
            }
        };

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
            styling::format_rune_group(&champ_runes[1].0.as_str())
        ]);
        rune_table.add_row(row![
            &champ_runes[0].1[0].rune.name,
            format!(
                "{} (Slot {})",
                &champ_runes[1].1[0].rune.name, &champ_runes[1].1[0].slot
            )
        ]);
        rune_table.add_row(row![
            &champ_runes[0].1[1].rune.name,
            format!(
                "{} (Slot {})",
                &champ_runes[1].1[1].rune.name, &champ_runes[1].1[1].slot
            )
        ]);
        rune_table.add_row(row![&champ_runes[0].1[2].rune.name]);
        rune_table.add_row(row![&champ_runes[0].1[3].rune.name]);
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

        if champ_overview.low_sample_size {
            println!();
            println!(
                " {} Data has a low sample size for this combination!",
                "Warning:".yellow().bold()
            );
        }

        println!();
    }
}
