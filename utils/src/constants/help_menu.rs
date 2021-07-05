use crate::constants::version::__VERSION__;
use colored::Colorize;

#[allow(unused)]
pub fn about() {
    let about: String = format!(
        r#"
Novus Package Manager {}

Usage: {} {} [<options>]

Commands:

  {} {} {} Installs packages.
  {} {} {} Uninstalls packages.
  {} {} {} Updates packages.
  {} {} {} Lists all packages.
  {} {} {} Searches for packages.  
  {} {} {} Provides information on a specific package.
  
Run {} for more info about each command."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "[command]".white(),
        "*".bright_magenta().bold(),
        "install".bright_cyan(),
        "(i)".yellow(),
        "*".bright_magenta().bold(),
        "uninstall".bright_cyan(),
        "(u)".yellow(),
        "*".bright_magenta().bold(),
        "update".bright_cyan(),
        "(upgrade)".yellow(),
        "*".bright_magenta().bold(),
        "list".bright_cyan(),
        "(show)".yellow(),
        "*".bright_magenta().bold(),
        "search".bright_cyan(),
        "(find)".yellow(),
        "*".bright_magenta().bold(),
        "info".bright_cyan(),
        "(details)".yellow(),
        "novus [command] --help".bright_green(),
    );

    println!("{}", about);
    std::process::exit(0);
}

#[allow(unused)]
pub fn install_help() {
    let init = format!(
        r#"
Novus Package Manager {}

Installs a package or a list of packages

Usage: {} {} {} {}
    
Options:
    
  {} {} Disable colored output for installation.  
  {} {} Disables progress bar for installation.
  {} {} Output verbose messages on internal operations."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "install".bright_purple(),
        "[package]".white(),
        "[flags]".white(),
        "--no-color".bright_cyan(),
        "(-nc)".yellow(),
        "--no-progress".bright_cyan(),
        "(-np)".yellow(),
        "--verbose".bright_cyan(),
        "(-v)".yellow(),
    );
    println!("{}", init);
    std::process::exit(0);
}

#[allow(unused)]
pub fn uninstall_help() {
    let install = format!(
        r#"
Novus Package Manager {}
    
Uninstall a package or a list of packages

Usage: {} {} {} {}
    
Options: 
    
  {} {} Disables progress bar for installation.
  {} {} Output verbose messages on internal operations."#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "uninstall".bright_purple(),
        "[package]".white(),
        "[flags]".white(),
        "--no-progress".bright_cyan(),
        "(-np)".yellow(),
        "--verbose".bright_cyan(),
        "(-v)".yellow()
    );
    println!("{}", install);
    std::process::exit(0);
}

#[allow(unused)]
pub fn update_help() {
    let update = format!(
        r#"
Novus Package Manager {}

Updates a package or a list of packages

Usage: {} {} {} {}

Options: 
    
  {} {} Disables progress bar for installation.
  {} {} Output verbose messages on internal operations."#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "add".bright_purple(),
        "[packages]".white(),
        "[flags]".white(),
        "--no-progress".bright_cyan(),
        "(-np)".yellow(),
        "--verbose".bright_cyan(),
        "(-v)".yellow(),
    );
    println!("{}", update);
    std::process::exit(0);
}

#[allow(unused)]
pub fn list_help() {
    let list = format!(
        r#"
Novus Package Manager {}

List all packages available.

Usage: {} {}

Options: 
  {} {} Displays all packages available
  {} {} Displays only installed packages
  {} {} Displays only local packages
  {} {} Displays only the package names"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "list".bright_purple(),
        "--all".bright_cyan(),
        "(-a)".yellow(),
        "--installed".bright_cyan(),
        "(-i)".yellow(),
        "--local".bright_cyan(),
        "(-l)".yellow(),
        "--names".bright_cyan(),
        "(-n)".yellow(),
    );
    println!("{}", list);
    std::process::exit(0);
}

#[allow(unused)]
pub fn info_help() {
    let info = format!(
        r#"
Novus Package Manager {}

Provides information on a specific package.

Usage: {} {}

Options: 
  {} {} Displays only local packages"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "list".bright_purple(),
        "--local".bright_cyan(),
        "(-l)".yellow(),
    );
    println!("{}", info);
    std::process::exit(0);
}

#[allow(unused)]
pub fn startup_help() {
    let startup = format!(
        r#"
Novus Package Manager {}

Allows you to modify starup apps

Usage: {} {} {} {}

Commands: 
  {} Lists all startup apps 
  {} Adds an app to startup
  {} Removes an app from startup

Options: 
  {} {} {} Displays help menu for a specific command"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "startup".bright_purple(),
        "[command]".white(),
        "[flags]".white(),
        "list".bright_cyan(),
        "add".bright_cyan(),
        "remove".bright_cyan(),
        "--help".bright_cyan(),
        "(-h)".yellow(),
        "(-?)".yellow(),
    );
    println!("{}", startup);
    std::process::exit(0);
}

#[allow(unused)]
pub fn search_help() {
    let search = format!(
        r#"
Novus Package Manager {}

Searches for a package based on input.

Usage: {} {} {}"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "search".bright_purple(),
        "7-zip".white(),
    );
    println!("{}", search);
    std::process::exit(0);
}

#[allow(unused)]
pub fn quit_help() {
    let quit = format!(
        r#"
Novus Package Manager {}

Quits an application or a list of applications.

Usage: {} {} {}

Options:
  {} {} Accepts all prompts
  {} {} Force quits the application"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "quit".bright_purple(),
        "7-zip".white(),
        "--yes".bright_cyan(),
        "(-y)".yellow(),
        "--force".bright_cyan(),
        "(-y)".yellow(),
    );
    println!("{}", quit);
    std::process::exit(0);
}

#[allow(unused)]
pub fn forcequit_help() {
    let forcequit = format!(
        r#"
Novus Package Manager {}

Force quits an application or a list of applications.

Short for {}

Usage: {} {} {}

Options:
  {} {} Accepts all prompts"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus quit -f".bright_green(),
        "novus".bright_green(),
        "quit".bright_purple(),
        "7-zip".white(),
        "--yes".bright_cyan(),
        "(-y)".yellow(),
    );
    println!("{}", forcequit);
    std::process::exit(0);
}

#[allow(unused)]
pub fn clean_help() {
    let clean = format!(
        r#"
Novus Package Manager {}

Clears novus cache

Usage: {} {} [packages] [flags]"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "clean".bright_purple(),
    );
    println!("{}", clean);
    std::process::exit(0);
}

#[allow(unused)]
pub fn install_error() {
    let install_error = format!(
        r#"
Novus Package Manager {}

{} Missing list of packages to install.
    
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus install --help".bright_green()
    );
    println!("{}", install_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn uninstall_error() {
    let uninstall_error = format!(
        r#"
Novus Package Manager {}

{} Missing list of packages to uninstall.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus uninstall --help".bright_green()
    );
    println!("{}", uninstall_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn search_error() {
    let search_error = format!(
        r#"
Novus Package Manager {}

{} Missing keyword to search for.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus search --help".bright_green()
    );
    println!("{}", search_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn startup_error() {
    let startup_error = format!(
        r#"
Novus Package Manager {}

{} Missing command to manage startup
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus startup --help".bright_green()
    );
    println!("{}", startup_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn startup_command_error() {
    let startup_command_error = format!(
        r#"
Novus Package Manager {}

{} Invalid command
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus startup --help".bright_green()
    );
    println!("{}", startup_command_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn startup_add_error() {
    let startup_command_error = format!(
        r#"
Novus Package Manager {}

{} Missing package to add
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus startup add --help".bright_green()
    );
    println!("{}", startup_command_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn startup_remove_error() {
    let startup_command_error = format!(
        r#"
Novus Package Manager {}

{} Missing package to remove
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus startup remove --help".bright_green()
    );
    println!("{}", startup_command_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn info_error() {
    let info_error = format!(
        r#"
Novus Package Manager {}

{} Missing package to provide information on.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus info --help".bright_green()
    );
    println!("{}", info_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn info_wrong_package_error() {
    let info_error = format!(
        r#"{}

Try running with the {} flag to provide information on packages which exist locally on your system.
      
{} Use {} for more information about this command."#,
        "Failed to locate package information.".bright_cyan(),
        "-l".bright_green(),
        "info".bright_blue(),
        "novus info --help".bright_green()
    );
    println!("{}", info_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn quit_error() {
    let quit_error = format!(
        r#"
Novus Package Manager {}

{} Missing application to quit.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus quit --help".bright_green()
    );
    println!("{}", quit_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn list_error(arg: &String) {
    let uninstall_error = format!(
        r#"
Novus Package Manager {}

{} Unexpected {}
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        arg,
        "info".bright_blue(),
        "novus list --help".bright_green()
    );
    println!("{}", uninstall_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn list_number_error() {
    let uninstall_error = format!(
        r#"
Novus Package Manager {}

{} Unexpected a number. Found a string instead.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus list --help".bright_green()
    );
    println!("{}", uninstall_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn invalid_command(command: &str) {
    println!(
        "{} {}\n{} Use {} for the list of all the commands\n",
        "error".bright_red(),
        format!("{} is not a valid command!", command.bright_blue()),
        "info".bright_blue(),
        "novus --help".bright_green()
    );
    std::process::exit(0);
}
