use colored::*;
use std::process::exit;

mod api;

enum ExitReasons {
    CouldNotGetVersion = 1,
}

fn log_error(msg: &str) {
    eprintln!("{} {}", "Error:".red().bold(), msg);
}

fn main() {
    let version = api::get_current_version();
    if version.is_none() {
        log_error("Could not get current patch version, exiting...");
        exit(ExitReasons::CouldNotGetVersion as i32);
    }
    println!(
        "Getting data for patch {}...",
        version.unwrap().green().bold()
    );
}
