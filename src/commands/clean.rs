use crate::utils::handle_error::handle_error_and_exit;
use std::path::Path;
use std::fs;
use colored::Colorize;
use crate::utils::cache::{clear_cache_for_package, clear_cache};

pub fn clean(args: Vec<String>) {
    let temp = std::env::var("TEMP").unwrap_or_else(|_| handle_error_and_exit("Failed to locate Temp directory".to_string()));
    let loc = format!("{}/novus", temp);
    let novus_dir = Path::new(&loc);
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