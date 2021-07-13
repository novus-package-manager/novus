use crate::constants::commands::{
    ALL_COMMANDS, CLEAN_FLAGS as clean_flags, FORCEQUIT_FLAGS as forcequit_flags,
    INFO_FLAGS as info_flags, INSTALL_FLAGS as install_flags, LIST_FLAGS as list_flags,
    QUIT_FLAGS as quit_flags, SEARCH_FLAGS as search_flags, UNINSTALL_FLAGS as uninstall_flags,
};
use crate::constants::help_menu::invalid_command;
use colored::Colorize;
use difflib::get_close_matches;
use std::{io::prelude::*, process};

pub fn verify_args(
    flags: Vec<String>,
    packages: Vec<String>,
    command: &str,
    package_list: Vec<&str>,
) -> (Vec<String>, Vec<String>) {
    let mut new_flags: Vec<String> = flags.clone();
    let mut new_packages: Vec<String> = packages.clone();

    if command != "search" && command != "list" && command != "info" && command != "startup" {
        new_packages = vec![];
        new_flags = vec![];
        for pkg in packages.iter() {
            let mut pkg_name = pkg.as_str();
            let mut version = "0";
            if pkg.contains("@") {
                let pkg_split: Vec<&str> = pkg.split("@").collect();
                pkg_name = pkg_split[0];
                version = pkg_split[1];
            };
            let revised_package = get_close_matches(pkg_name, package_list.clone(), 1, 0.6);
            if package_list.contains(&pkg_name) {
                new_packages.push(pkg.to_string().clone());
            } else if revised_package.len() == 1 {
                print!(
                    "Could not find {} package. {} {} instead? (Y/N): ",
                    pkg_name.bright_magenta(),
                    command,
                    revised_package[0].bright_green()
                );
                std::io::stdout()
                    .flush()
                    .ok()
                    .expect("Could not flush stdout");
                let mut string: String = String::new();
                let _ = std::io::stdin().read_line(&mut string);
                if string.trim().to_lowercase() == "y" {
                    let new_package = revised_package[0].to_string() + "@" + version;
                    new_packages.push(new_package.to_string());
                } else {
                    process::exit(0);
                }
            } else {
                println!(
                    "Package {} doesn't exist yet :(\n\n{} run {} to view all available packages.",
                    pkg_name.bright_magenta(),
                    "info:".bright_blue(),
                    "novus list".bright_magenta()
                );
                std::process::exit(1)
            }
        }
    }

    match command {
        "install" => {
            for flag in flags.iter() {
                for install_flag in install_flags.iter() {
                    if install_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
                }
            }
        }
        "uninstall" => {
            for flag in flags.iter() {
                for uninstall_flag in uninstall_flags.iter() {
                    if uninstall_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
                }
            }
        }
        "update" => {
            for flag in flags.iter() {
                for install_flag in install_flags.iter() {
                    if install_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
                }
            }
        }
        "list" => {
            for flag in flags.iter() {
                for list_flag in list_flags.iter() {
                    if list_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
                }
            }
        }
        "info" => {
            for flag in flags.iter() {
                for info_flag in info_flags.iter() {
                    if info_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
                }
            }
        }
        "clean" => {
            for flag in flags.iter() {
                for clean_flag in clean_flags.iter() {
                    if clean_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
                }
            }
        }
        "search" => {
            for flag in flags.iter() {
                for search_flag in search_flags.iter() {
                    if search_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
                }
            }
        }
        "quit" => {
            for flag in flags.iter() {
                for quit_flag in quit_flags.iter() {
                    if quit_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
                }
            }
        }
        "forcequit" => {
            for flag in flags.iter() {
                for forcequit_flag in forcequit_flags.iter() {
                    if forcequit_flag.contains(&flag.as_str()) {
                        new_flags.push(flag.clone());
                    }
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
        if ALL_COMMANDS.contains(&command) {
            if args[arg].starts_with("-") {
                flags.push(args[arg].clone());
            } else {
                packages.push(args[arg].clone().to_lowercase());
            }
        } else {
            invalid_command(command);
        }
    }

    (flags, packages)
}
