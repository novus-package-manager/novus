use utils::constants::commands::CONFIG_FLAGS;
use utils::constants::config_menu::*;
use std::process;

#[allow(unused)]
pub async fn config(args: Vec<String>, flags: Vec<String>) {
    let (command, value) = parse_args(args);    
    
    // Handle help menus
    handle_help_menu(&command, flags);

    // Validate value
    validate_value(&value);

    process::exit(0)
}

fn validate_value(value: &str) {
    if value == "true" || value == "false" {
        return;
    }
    else {
        config_error_value();
        process::exit(1);
    }
}

fn handle_help_menu(command: &str, flags: Vec<String>) {
    if flags.contains(&"-h".to_string()) || flags.contains(&"-?".to_string()) || flags.contains(&"--help".to_string()) {
        if command == "multithreaded" {
            config_multithreaded_help();        
        }
        if command == "no-color" {
            config_no_color_help();        
        }
        if command == "no-progress" {
            config_no_progress_help();        
        }
        process::exit(0);
    }
}

fn parse_args(args: Vec<String>) -> (String, String) {
    let command = &args[2];
    let mut value = "";

    if !CONFIG_FLAGS.contains(&command.as_str()) {
        config_error_value();
        process::exit(1);
    }

    if args.len() <= 3 {
        config_error_flag();
    } else {
        value = &args[3];
    }

    (command.to_owned(), value.to_string())
}
