#[path = "../constants/help_menu.rs"]
mod help_menu;

use help_menu::{ about, install_help, uninstall_help, update_help, list_help, install_error, uninstall_error, invalid_command };
use colored::Colorize;

const __VERSION__: &str = "v1.0.0";

pub fn display_help(args: &Vec<String>) -> &String {
  if args.len() == 1 {
      about();
  }
  else if args.len() == 2 {
      let command: &str = args[1].as_str();
      match command {
          "--version" => println!("{}", format!("volt {}", __VERSION__.bright_green().bold())),
          "install" => install_error(),
          "uninstall" => uninstall_error(),
          "update" => {},
          "list" => {},
          "--help" => about(),
          &_ => invalid_command(command)
      }        
  }
  else if args.len() > 2 {
      let command: &str = args[1].as_str();
      if args[2].as_str().starts_with("--") {
          let flag: &str = args[2].as_str();      
          if flag == "--help" {
              match command {
                  "install" => install_help(),
                  "uninstall" => uninstall_help(),
                  "update" => update_help(),
                  "list" => list_help(),
                  &_ => invalid_command(command)
              }        
          }
      }      
  }

  &args[1]
}