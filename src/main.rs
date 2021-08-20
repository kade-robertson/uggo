use chrono;
use colored::*;
use std::process::exit;

mod api;

enum ExitReasons {
    CouldNotGetVersion = 1,
    CouldNotGetChampData = 2,
    CouldNotGetItemData = 3,
    CouldNotGetRuneData = 4,
}

static TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

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
}
