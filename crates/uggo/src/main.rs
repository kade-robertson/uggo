#![deny(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[macro_use]
extern crate prettytable;

use colored::Colorize;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use league_client_connector::LeagueClientConnector;

use anyhow::Result;
use bpaf::Bpaf;
use prettytable::{format, Table};
use std::io;
use std::io::Write;
use text_io::read;
use ugg_types::mappings::{self, Mode};

use crate::styling::format_ability_order;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use ugg_types::client_runepage::NewRunePage;

#[cfg(any(target_os = "windows", target_os = "macos"))]
mod client_api;

use uggo_ugg_api::UggApi;

mod styling;
mod util;

fn fetch(
    ugg: &UggApi,
    #[cfg(any(target_os = "windows", target_os = "macos"))] client: &Option<client_api::ClientAPI>,
    #[cfg(not(any(target_os = "windows", target_os = "macos")))] _client: Option<bool>,
    champ: &str,
    mode: mappings::Mode,
    role: mappings::Role,
    region: mappings::Region,
) {
    let query_champ = ugg.find_champ(champ);

    let formatted_champ_name = query_champ.name.as_str().green().bold();

    let mut query_message = vec![format!("Looking up info for {formatted_champ_name}")];
    if role != mappings::Role::default() {
        query_message.push(format!(", playing {}", role.to_string().blue().bold()));
    }
    if region != mappings::Region::default() {
        query_message.push(format!(", in {}", region.to_string().red().bold()));
    }
    query_message.push("...".to_string());
    util::log_info(query_message.concat().as_str());

    let champ_overview = if let Ok(data) = ugg.get_stats(query_champ, role, region, mode) {
        *data
    } else {
        util::log_error(format!("Couldn't get required data for {formatted_champ_name}.").as_str());
        return;
    };

    let matchups = if mode == Mode::ARAM {
        None
    } else {
        ugg.get_matchups(query_champ, role, region, mode).ok()
    };

    let stats_message = vec![format!("Build for {formatted_champ_name}")];
    let true_length = 10 /* "Build for " */ + query_champ.name.len();
    let stats_message_str = stats_message.concat();
    println!(" {}", "-".repeat(true_length));
    println!(" {stats_message_str}");
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
    for shard in &util::process_shards(&champ_overview.shards.shard_ids) {
        println!(" {shard}");
    }

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

    if let Some(safe_matchups) = matchups {
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

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    if let Some(ref api) = client {
        if let Some(data) = api.get_current_rune_page() {
            let (primary_style_id, sub_style_id, selected_perk_ids) =
                util::generate_perk_array(&champ_runes, &champ_overview.shards.shard_ids);
            api.update_rune_page(
                data.id,
                &NewRunePage {
                    name: match mode {
                        mappings::Mode::ARAM => {
                            format!("uggo: {}, ARAM", &query_champ.name)
                        }
                        mappings::Mode::URF => format!("uggo: {}, URF", &query_champ.name),
                        _ => format!("uggo: {}, Normal", &query_champ.name),
                    },
                    primary_style_id,
                    sub_style_id,
                    selected_perk_ids,
                },
            );
        }
    }
}

#[derive(Debug, Bpaf)]
#[bpaf(options, version)]
struct Options {
    /// The game mode to look for data from. By default, this is set to Normal.
    ///
    /// Can be one of: normal, aram, urf, arurf, oneforall
    #[bpaf(short('m'), long)]
    mode: Option<mappings::Mode>,

    /// Can be specified to pull build data for a specific role. By default, this will
    /// not be necessary as the most popular role will be picked automatically. In ARAM,
    /// this setting is ignored.
    ///
    /// Can be one of: top, mid, bottom, adc, jungle, none, automatic
    #[bpaf(short('r'), long)]
    role: Option<mappings::Role>,

    /// The region to use to filter build results. By default, this uses all regions.
    ///
    /// Can be one of: NA1, EUW1, KR, EUN1, BR1, LA1, LA2, OC1, RU, TR1, JP1, World, PH2, SG2, TH2, TW2, VN2
    #[bpaf(short('R'), long)]
    region: Option<mappings::Region>,

    /// The ddragon API version to use instead of the latest version. Useful for when
    /// u.gg does not have builds for the current patch, for example.
    #[bpaf(short('v'), long)]
    api_version: Option<String>,

    /// The name of the champion you want to match. A best effort will be made
    /// to find the champ if it's only a partial query.
    ///
    /// If left blank, will open the interactive version of uggo.
    #[bpaf(positional("CHAMP"))]
    champ: Option<String>,
}

fn main() -> Result<()> {
    let parsed_args = options().run();

    let ugg = UggApi::new(parsed_args.api_version.clone())?;

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let client_lockfile = LeagueClientConnector::parse_lockfile().ok();

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let mut clientapi: Option<client_api::ClientAPI> = None;

    #[cfg(all(debug_assertions, any(target_os = "windows", target_os = "macos")))]
    if client_lockfile.as_ref().is_some() {
        let lockfile = client_lockfile.clone().unwrap();
        util::log_info(&format!(
            "- Found client running at https://127.0.0.1:{}/",
            lockfile.port
        ));
        clientapi = Some(client_api::ClientAPI::new(lockfile));
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    if client_lockfile.as_ref().is_some() {
        clientapi = Some(client_api::ClientAPI::new(client_lockfile.unwrap()));
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    if let Some(ref api) = clientapi {
        if let Some(summoner) = api.get_summoner_info() {
            #[cfg(debug_assertions)]
            util::log_info(&format!(
                "- Found summoner {} (id: {})",
                summoner.display_name, summoner.summoner_id
            ));
        }
    }

    if let Some(champ_name) = parsed_args.champ {
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        fetch(
            &ugg,
            None,
            &champ_name,
            parsed_args.mode.unwrap_or_default(),
            parsed_args.role.unwrap_or_default(),
            parsed_args.region.unwrap_or_default(),
        );

        #[cfg(any(target_os = "windows", target_os = "macos"))]
        fetch(
            &ugg,
            &clientapi,
            &champ_name,
            parsed_args.mode.unwrap_or_default(),
            parsed_args.role.unwrap_or_default(),
            parsed_args.region.unwrap_or_default(),
        );
        return Ok(());
    }

    let mut mode = mappings::Mode::Normal;
    loop {
        print!("query> ");
        io::stdout().flush().unwrap();
        let user_input: String = read!("{}\n");
        let clean_user_input = user_input.trim();
        let user_input_split = clean_user_input
            .split(',')
            .map(str::trim)
            .collect::<Vec<&str>>();

        if clean_user_input == "modes" {
            util::log_info("Available modes:");
            mappings::Mode::all()
                .iter()
                .for_each(|m| util::log_info(format!("- {m:?}").as_str()));
            continue;
        }

        if clean_user_input.starts_with("mode") {
            let mode_to_set = clean_user_input.split(' ').collect::<Vec<&str>>();
            if mode_to_set.len() > 1 {
                mode = mappings::Mode::from(mode_to_set[1]);
                util::log_info(format!("Switching mode to {mode:?}...").as_str());
                continue;
            }
            util::log_info(format!("Current mode: {mode:?}").as_str());
            continue;
        }

        let mut query_champ_name = "";
        let mut query_role = mappings::Role::default();
        let mut query_region = mappings::Region::default();

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

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        fetch(&ugg, None, query_champ_name, mode, query_role, query_region);

        #[cfg(any(target_os = "windows", target_os = "macos"))]
        fetch(
            &ugg,
            &clientapi,
            query_champ_name,
            mode,
            query_role,
            query_region,
        );

        println!();
    }
}
