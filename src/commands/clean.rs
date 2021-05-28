use colored::Colorize;
use crate::utils::cache::{clear_cache_for_package, clear_cache};

pub fn clean(args: Vec<String>) {
    let mut packages = vec![];
    let mut flags = vec![];
    for arg in args.clone() {
        if arg.starts_with("-") || arg.starts_with("--") {
            flags.push(arg);
        }
        else {
            if arg != "clean" || arg != "novus" {
                packages.push(arg);
            }
        }
    }
    println!("{}", "Clearing Cache".bright_green());
    if args.len() == 2 {
        clear_cache();
    }
    else {
        for package in packages {
            clear_cache_for_package(&package);
        }
    }

    println!("{}", "Successfully Cleared Cache".bright_purple());
}