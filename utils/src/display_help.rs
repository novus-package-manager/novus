use crate::check_version::check_version;
use crate::constants::commands::COMMANDS;
use crate::constants::help_menu::{
    about, clean_help, display_version, forcequit_help, info_error, info_help, install_error,
    install_help, invalid_command, list_help, quit_error, quit_help, search_error, search_help,
    startup_error, startup_help, uninstall_error, uninstall_help, update_error, update_help, config_error, config_help
};

use colored::Colorize;

pub async fn display_help(args: &Vec<String>) -> &String {
    if args.len() == 1 {
        about();

        let update_available = check_version().await;

        if update_available {
            println!(
                "\n{} A new version of Novus is available. Run {} to update.",
                "NOTE".bright_cyan(),
                "novus update novus".bright_cyan()
            );
        }
        
        std::process::exit(0);
    } else if args.len() == 2 {
        let mut command: &str = args[1].as_str();
        for cmd in COMMANDS.iter() {
            if command == cmd[1] {
                command = cmd[0];
            }
        }
        match command {
            "--version" => display_version(),
            "-v" => display_version(),
            "install" => install_error(),
            "uninstall" => uninstall_error(),
            "update" => update_error(),
            "list" => {}
            "clean" => {}
            "search" => search_error(),
            "quit" => quit_error(),
            "forcequit" => quit_error(),
            "info" => info_error(),
            "startup" => startup_error(),
            "config" => config_error(),
            "--help" => about(),
            "-h" => about(),
            "-?" => about(),
            &_ => invalid_command(command),
        }

        let update_available = check_version().await;

        if update_available {
            println!(
                "\n{} A new version of Novus is available. Run {} to update.",
                "NOTE".bright_cyan(),
                "novus update novus".bright_cyan()
            );
        }

        if command != "clean" && command != "list" {
            std::process::exit(0);
        }
    } else if args.len() > 2 {
        let mut command: &str = args[1].as_str();
        for cmd in COMMANDS.iter() {
            if command == cmd[1] {
                command = cmd[0];
            }
        }
        if args[2].as_str().starts_with("-") {
            let flag: &str = args[2].as_str();
            if flag == "--help" || flag == "-?" || flag == "-h" {
                match command {
                    "install" => install_help(),
                    "uninstall" => uninstall_help(),
                    "update" => update_help(),
                    "list" => list_help(),
                    "search" => search_help(),
                    "quit" => quit_help(),
                    "forcequit" => forcequit_help(),
                    "clean" => clean_help(),
                    "info" => info_help(),
                    "startup" => startup_help(),
                    "config" => config_help(),
                    &_ => invalid_command(command),
                }

                let update_available = check_version().await;

                if update_available {
                    println!(
                        "\n{} A new version of Novus is available. Run {} to update.",
                        "NOTE".bright_cyan(),
                        "novus update novus".bright_cyan()
                    );
                }

                std::process::exit(0);
            }
        }
    }

    &args[1]
}
