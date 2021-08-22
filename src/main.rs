use chrono;
use colored::*;
use ctrlc;
use std::io;
use std::io::Write;
use std::process::exit;
use text_io::read;

mod api;
mod mappings;
mod styling;
mod util;

enum ExitReasons {
    Neutral = 0,
    CouldNotGetVersion,
    CouldNotGetChampData,
    CouldNotGetItemData,
    CouldNotGetRuneData,
}

static TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";
static DEFAULT_ROLE: mappings::Role = mappings::Role::Automatic;
static DEFAULT_REGION: mappings::Region = mappings::Region::NA1;

fn log_error(msg: &str) {
    let now = chrono::Local::now();
    eprintln!(
        "[{}] {} {}",
        now.format(TIME_FORMAT),
        "Error:".red().bold(),
        msg
    );
}

fn log_info(msg: &str) {
    let now = chrono::Local::now();
    println!("[{}] {}", now.format(TIME_FORMAT), msg);
}

fn main() {
    ctrlc::set_handler(move || {
        println!("\nExiting...");
        exit(ExitReasons::Neutral as i32);
    })
    .expect("Couldn't handle Ctrl+C");

    let version = api::get_current_version();
    if version.is_none() {
        log_error("Could not get current patch version, exiting...");
        exit(ExitReasons::CouldNotGetVersion as i32);
    }
    let safe_version = version.clone().unwrap();
    log_info(&format!(
        "Getting data for patch {}...",
        safe_version.green().bold()
    ));

    let champ_data = api::get_champ_data(&safe_version);
    if champ_data.is_none() {
        log_error("Could not download champ data, exiting...");
        exit(ExitReasons::CouldNotGetChampData as i32);
    }
    log_info(&format!(
        "- Got data for {} champ(s).",
        champ_data
            .clone()
            .unwrap()
            .keys()
            .len()
            .to_string()
            .green()
            .bold()
    ));

    let item_data = api::get_items(&safe_version);
    if item_data.is_none() {
        log_error("Could not download item data, exiting...");
        exit(ExitReasons::CouldNotGetItemData as i32);
    }
    log_info(&format!(
        "- Got data for {} items(s).",
        item_data
            .clone()
            .unwrap()
            .keys()
            .len()
            .to_string()
            .green()
            .bold()
    ));

    let rune_data = api::get_runes(&safe_version);
    if rune_data.is_none() {
        log_error("Could not download rune data, exiting...");
        exit(ExitReasons::CouldNotGetRuneData as i32);
    }
    log_info(&format!(
        "- Got data for {} rune(s).",
        rune_data
            .clone()
            .unwrap()
            .keys()
            .len()
            .to_string()
            .green()
            .bold()
    ));

    let mut patch_version_split = safe_version.split(".").collect::<Vec<&str>>();
    patch_version_split.remove(patch_version_split.len() - 1);
    let patch_version = patch_version_split.join("_");

    loop {
        print!("query> ");
        io::stdout().flush().unwrap();
        let user_input: String = read!("{}\n");
        let user_input_split = user_input.split(',').collect::<Vec<&str>>();

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

        let cloned_champ_data = &champ_data.clone().unwrap();
        let query_champ = util::find_champ(query_champ_name, cloned_champ_data);

        let formatted_champ_name = query_champ["name"].as_str().unwrap().green().bold();

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

        let champ_stats = api::get_stats(
            &patch_version.as_str(),
            query_champ,
            query_role,
            query_region,
        );
        if champ_stats.is_none() {
            log_error(format!("Couldn't get required data for {}.", formatted_champ_name).as_str());
            continue;
        }

        let (overview_role, champ_overview) = champ_stats.unwrap();
        let mut stats_message = vec![format!("Build for {}", formatted_champ_name)];
        let mut true_length = 10 /* "Build for " */ + query_champ["name"].as_str().unwrap().len();
        if overview_role != mappings::Role::None {
            stats_message.push(format!(
                ", playing {} lane",
                overview_role.to_string().blue().bold()
            ));
            true_length += 15 /* ", playing  lane" */ + overview_role.to_string().len();
        }
        let stats_message_str = stats_message.concat();
        println!("{}", "-".repeat(true_length));
        println!("{}", stats_message_str);
        println!("{}", "-".repeat(true_length));

        let champ_runes = util::group_runes(
            &champ_overview[0][0][4].as_array().unwrap(),
            &rune_data.as_ref().unwrap(),
        );
        println!("Runes");
        println!("-----");
        println!("{}", styling::format_rune_group(&champ_runes[0].0.as_str()));
        champ_runes[0].1.iter().for_each(|r| println!("- {}", r));
        println!("{}", styling::format_rune_group(&champ_runes[1].0.as_str()));
        champ_runes[1].1.iter().for_each(|r| println!("- {}", r));
    }
}
