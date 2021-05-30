#![cfg(windows)]

mod classes;
mod commands;
mod constants;
mod utils;

use colored::Colorize;
use commands::{clean, install, list, quit, search, uninstall};
use display_help::display_help;

use clean::clean;
use constants::commands::COMMANDS;
use handle_args::{get_arguments, verify_args};
use install::installer;
use list::list;
use quit::quit;
use search::search;
use serde_json::Value;
use uninstall::uninstaller;
use utils::{display_help, get_package, handle_args, handle_error::handle_error_and_exit};
// use std::time::Instant;

#[allow(unused)]
#[tokio::main]
async fn main() {
    let _enabled = ansi_term::enable_ansi_support();

    create_dirs();

    ctrlc::set_handler(move || {
        println!("\n{}", "Aborted!".bright_blue());
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let args: Vec<String> = std::env::args().collect();

    let command = display_help(&args);

    #[allow(unused)]
    let mut data = String::new();

    data = get_package::get_packages().await;

    let val = data
        .as_str()
        .parse::<Value>()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let package_list: Vec<&str> = val["packages"]
        .as_array()
        .unwrap_or_else(|| handle_error_and_exit("An error occured".to_string()))
        .iter()
        .map(|p| {
            p.as_str()
                .unwrap_or_else(|| handle_error_and_exit("An error occured".to_string()))
        })
        .collect();

    let mut command: &str = command;

    for cmd in COMMANDS.iter() {
        if command == cmd[1] {
            command = cmd[0];
        }
    }

    let (flags, packages) = get_arguments(&args);

    let (flags, packages) = verify_args(
        flags.clone(),
        packages.clone(),
        command,
        package_list.clone(),
    );

    match command {
        "install" => {
            installer(packages, flags).await;
        }
        "uninstall" => {
            uninstaller(packages).await;
        }
        "update" => {
            installer(packages, flags).await;
        }
        "list" => {
            list(package_list, flags).await;
        }
        "clean" => {
            clean(args);
        }
        "search" => {
            search(package_list, flags, &packages[0]).await;
        }
        "quit" => {
            quit(packages, flags, false).await;
        }
        "forcequit" => {
            quit(packages, flags, true).await;
        }
        &_ => {}
    }
}

fn create_dirs() {
    let temp = std::env::var("TEMP").unwrap();
    let loc = format!(r"{}\novus\", temp);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }
    let loc = format!(r"{}\novus\config", temp);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }
}
