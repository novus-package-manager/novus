#[path = "../constants/commands.rs"]
mod commands;

use colored::Colorize;
use difflib::get_close_matches;

use std::{io::prelude::*, process};

use commands::{
    INSTALL_FLAGS as install_flags, LIST_FLAGS as list_flags, UNINSTALL_FLAGS as uninstall_flags,
};

pub fn verify_args(
    flags: Vec<String>,
    packages: Vec<String>,
    command: &String,
    package_list: Vec<&str>,
) -> (Vec<String>, Vec<String>) {
    let mut new_flags: Vec<String> = vec![];
    let mut new_packages: Vec<String> = vec![];

    for pkg in packages.iter() {
        let revised_package = get_close_matches(pkg, package_list.clone(), 1, 0.6);
        if package_list.contains(&pkg.as_str()) {
            new_packages.push(pkg.clone());
        } else if revised_package.len() == 1 {
            print!(
                "Could not find {} package. Install {} instead? (Y/N): ",
                pkg.bright_magenta(),
                revised_package[0].bright_green()
            );
            std::io::stdout()
                .flush()
                .ok()
                .expect("Could not flush stdout");
            let mut string: String = String::new();
            let _ = std::io::stdin().read_line(&mut string);
            if string.trim().to_lowercase() == "y" {
                new_packages.push(revised_package[0].to_string());
            } else {
                process::exit(0);
            }
        } else {
            println!(
                "Package {} doesn't exist yet :(\n\n{} run {} to view all available packages.",
                pkg.bright_magenta(),
                "info:".bright_blue(),
                "novus list".bright_magenta()
            );
            std::process::exit(1)
        }
    }

    match command.as_str() {
        "install" => {
            for flag in flags.iter() {
                if install_flags.contains(&flag.as_str()) {
                    new_flags.push(flag.clone());
                }
            }
        }
        "uninstall" => {
            for flag in flags.iter() {
                if uninstall_flags.contains(&flag.as_str()) {
                    new_flags.push(flag.clone());
                }
            }
        }
        "list" => {
            for flag in flags.iter() {
                if list_flags.contains(&flag.as_str()) {
                    new_flags.push(flag.clone());
                }
            }
        }
        &_ => {}
    }

    (new_flags, new_packages)
}

pub fn get_arguments(args: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let command: &str = &args[1];
    let mut flags: Vec<String> = vec![];
    let mut packages: Vec<String> = vec![];

    for arg in 2..args.len() {
        if command == "install"
            || command == "uninstall"
            || command == "update"
            || command == "list"
            || command == "auto"
        {
            if args[arg].starts_with("--") {
                flags.push(args[arg].clone());
            } else {
                packages.push(args[arg].clone().to_lowercase());
            }
        }
    }

    (flags, packages)
}
