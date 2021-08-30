use utils::constants::commands::CONFIG_FLAGS;
use utils::handle_error::handle_error_and_exit;
use utils::constants::config_menu::*;
use std::process;
use std::path::Path;
use std::env;
use std::fs::File;
use utils::classes::config::Config;
use colored::Colorize;

#[allow(unused)]
pub async fn config(args: Vec<String>, flags: Vec<String>) {
    let (command, old_value) = parse_args(args);
    
    // Handle help menus
    handle_help_menu(&command, flags);

    // Validate value
    let mut value = false;
    if command != "reset" && command != "default" && command != "list"
    {
        value = validate_value(&old_value);
    }   

    // Get Config File
    let (mut config, file) = get_config();

    // Edit Config
    if command == "multithreaded" {
        config.multithreaded = value;
    }
    if command == "no-color" {
        config.no_color = value;
    }
    if command == "no-progress" {
        config.no_progress = value;
    }
    if command == "portable" {
        config.portable = value;
    }
    if command == "confirm" {
        config.confirm = value;
    }

    // Reset config to defaults
    if command == "reset" || command == "default" {
        config.multithreaded = false;
        config.no_color = false;
        config.no_progress = false;
        config.portable = false;
        config.confirm = false;
    }

    // List config
    if command == "list" {
        println!("{}{}", "multithreaded: ".bright_cyan(), config.multithreaded);
        println!("{}{}", "no-color: ".bright_cyan(), config.no_color);
        println!("{}{}", "no-progress: ".bright_cyan(), config.no_progress);
        println!("{}{}", "portable: ".bright_cyan(), config.portable);
        println!("{}{}", "confirm: ".bright_cyan(), config.confirm);
    }

    // Write Config File
    serde_json::to_writer_pretty(file, &config).unwrap_or_else(|_| handle_error_and_exit("Failed to write config file".to_string()));

    // Confirmation Message
    if command != "list" {
        println!("{}", "Successfully Updated Configuration".bright_green());
    }

    process::exit(0)
}

fn get_config() -> (Config, File) {
    let appdata = env::var("APPDATA").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate appdata directory".to_string())
    });
    let loc = format!(r"{}\novus\config\config.json", appdata);
    let path = Path::new(loc.as_str());
    let contents = std::fs::read_to_string(path).unwrap_or_else(|_| handle_error_and_exit("Failed to open config file".to_string()));
    let config: Config = serde_json::from_str::<Config>(&contents).unwrap_or_else(|_| handle_error_and_exit("Failed to parse config file".to_string()));

    let file = File::create(path).unwrap_or_else(|_| handle_error_and_exit("Failed to open config file".to_string()));

    (config, file)
}

fn validate_value(value: &str) -> bool {
    let value = value.to_lowercase();
    if value == "true" || value == "yes" {
        return true;
    }
    else if value == "false" || value == "no" {
        return false;
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
        if command == "portable" {
            config_portable_help();        
        }
        if command == "confirm" {
            config_confirm_help();       
        }
        if command == "reset" {
            config_reset_help();
        }
        if command == "default" {
            config_reset_help();     
        }
        process::exit(0);
    }
}

fn parse_args(args: Vec<String>) -> (String, String) {
    let command = &args[2].to_lowercase();
    let mut value = "";

    if !CONFIG_FLAGS.contains(&command.as_str()) {
        config_error_value();
        process::exit(1);
    }

    if command != "reset" && command != "default" && command != "list"
    {
        if args.len() <= 3 {
            config_error_flag();
            process::exit(1);
        } else {
            value = &args[3];
        }
    } 

    (command.to_owned(), value.to_string())
}
