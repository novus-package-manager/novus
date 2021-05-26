mod classes;
mod commands;
mod constants;
mod utils;

use colored::Colorize;
use commands::{install, uninstall};
use display_help::display_help;
use handle_args::{get_arguments, verify_args};
use handle_error::handle_error_and_exit;
use install::installer;
use serde_json::Value;
use uninstall::uninstaller;
use utils::{display_help, get_package, handle_args, handle_error};
// use std::time::Instant;

#[allow(unused)]
#[tokio::main]
#[cfg(windows)]
async fn main() {
    let _ = ansi_term::enable_ansi_support();

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

    let (flags, packages) = get_arguments(&args);

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

    let (flags, packages) = verify_args(flags, packages, command, package_list.clone());

    match command.as_str() {
        "install" => {
            installer(packages, flags).await;
        }
        "uninstall" => {
            uninstaller(packages).await;
        }
        "list" => {
            constants::help_menu::list_packages(package_list);
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
}
