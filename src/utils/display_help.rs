use crate::constants::help_menu::{ about, install_help, uninstall_help, update_help, list_help, install_error, uninstall_error, invalid_command, clean_help, search_help };
use crate::constants::version::__VERSION__;
use crate::constants::commands::COMMANDS;
use colored::Colorize;

pub fn display_help(args: &Vec<String>) -> &String {
  if args.len() == 1 {
      about();
  }
  else if args.len() == 2 {
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
          "update" => {},
          "list" => {},
          "clean" => {},
          "search" => {},
          "--help" => about(),
          "-h" => about(),
          "-?" => about(),
          &_ => invalid_command(command)
      }
  }
  else if args.len() > 2 {
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
                  "clean" => clean_help(),
                  &_ => invalid_command(command)
              }        
          }
      }      
  }

  &args[1]
}