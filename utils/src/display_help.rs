use crate::constants::commands::COMMANDS;
use crate::constants::help_menu::{
    about, clean_help, forcequit_help, info_error, info_help, install_error, install_help,
    invalid_command, list_help, quit_error, quit_help, search_error, search_help, uninstall_error,
    uninstall_help, update_help, startup_error, startup_help
};
use crate::constants::version::__VERSION__;
use colored::Colorize;

pub fn display_help(args: &Vec<String>) -> &String {
    if args.len() == 1 {
        about();
    } else if args.len() == 2 {
        let mut command: &str = args[1].as_str();
        for cmd in COMMANDS.iter() {
            if command == cmd[1] {
                command = cmd[0];
            }
        }
        match command {
            "--version" => println!("{}", format!("volt {}", __VERSION__.bright_green().bold())),
            "install" => install_error(),
            "uninstall" => uninstall_error(),
            "update" => {}
            "list" => {}
            "clean" => {}
            "search" => search_error(),
            "quit" => quit_error(),
            "forcequit" => quit_error(),
            "info" => info_error(),
            "startup" => startup_error(),
            "--help" => about(),
            "-h" => about(),
            "-?" => about(),
            &_ => invalid_command(command),
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
                    &_ => invalid_command(command),
                }
            }
        }
    }

    &args[1]
}
