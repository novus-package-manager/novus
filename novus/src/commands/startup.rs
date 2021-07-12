use colored::Colorize;
use utils::constants::help_menu::{startup_add_error, startup_command_error, startup_remove_error};
use utils::registry::get_startup_apps;

pub async fn startup(args: Vec<String>, _flags: Vec<String>) {
    let (command, _package_name) = parse_args(args);

    // Handle list command
    if command == "list" {
        let apps = get_startup_apps();
        for app in apps {
            println!("{}{}", " - ".bright_blue(), app.bright_cyan());
        }
    }
}

const COMMANDS: [&str; 3] = ["list", "add", "remove"];

fn parse_args(args: Vec<String>) -> (String, String) {
    let command = &args[2];
    let mut package_name = "";

    if !COMMANDS.contains(&command.as_str()) {
        startup_command_error();
    }

    if command == "add" || command == "remove" {
        if args.len() <= 3 {
            if command == "add" {
                startup_add_error();
            } else {
                startup_remove_error();
            }
        } else {
            package_name = &args[3];
        }
    }

    (command.to_owned(), package_name.to_string())
}
