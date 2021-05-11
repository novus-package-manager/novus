#[path = "../utils/display_help.rs"]
mod display_help;

#[path = "../utils/handle_args.rs"]
mod handle_args;

#[path = "../commands/install.rs"]
mod install;

#[path = "../commands/uninstall.rs"]
mod uninstall;

#[path = "../utils/get_package.rs"]
mod get_package;

#[path = "../utils/handle_error.rs"]
mod handle_error;

use colored::Colorize;
use display_help::display_help;
use handle_args::{get_arguments, verify_args};
use handle_error::handle_error_and_exit;
use install::installer;
use serde_json::Value;
// use std::time::Instant;
use uninstall::uninstaller;

#[allow(unused)]
fn main() {
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

    data = get_package::get_packages();

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

    let (flags, packages) = verify_args(flags, packages, command, package_list);

    match command.as_str() {
        "install" => {
            installer(packages);
        }
        "uninstall" => {
            uninstaller(packages);
            // println!("total time: {:?}", start.elapsed());
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
