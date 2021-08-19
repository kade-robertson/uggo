use chrono;
use colored::*;
use std::process::exit;

mod api;

enum ExitReasons {
    CouldNotGetVersion = 1,
    CouldNotGetChampData = 2,
}

fn log_error(msg: &str) {
    let now = chrono::Local::now();
    eprintln!(
        "[{}] {} {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        "Error:".red().bold(),
        msg
    );
}

fn log_info(msg: &str) {
    let now = chrono::Local::now();
    println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), msg);
}

fn main() {
    let version = api::get_current_version();
    if version.is_none() {
        log_error("Could not get current patch version, exiting...");
        exit(ExitReasons::CouldNotGetVersion as i32);
    }
    log_info(&format!(
        "Getting data for patch {}...",
        version.clone().unwrap().green().bold()
    ));
    let champ_data = api::get_champ_data(version.clone().unwrap());
    if champ_data.is_none() {
        log_error("Could not download champ data, exiting...");
        exit(ExitReasons::CouldNotGetChampData as i32);
    }
    log_info(&format!(
        "Got champ data for {} champ(s).",
        champ_data
            .clone()
            .unwrap()
            .keys()
            .len()
            .to_string()
            .green()
            .bold()
    ));
    let rune_data = api::get_runes(version.clone().unwrap());
    log_info(&format!(
        "Got rune data for {} rune(s).",
        rune_data
            .clone()
            .unwrap()
            .keys()
            .len()
            .to_string()
            .green()
            .bold()
    ));
}
