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
  {} {} {} Clears all cache.
  {} {} {} Quits an application or a list of applications.
  {} {} {} Forcequits an application or a list of applications.
  
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
        "*".bright_magenta().bold(),
        "clean".bright_cyan(),
        "(clear)".yellow(),
        "*".bright_magenta().bold(),
        "quit".bright_cyan(),
        "(exit)".yellow(),
        "*".bright_magenta().bold(),
        "forcequit".bright_cyan(),
        "(forcequit)".yellow(),
        "novus [command] --help".bright_green(),
    );

    println!("{}", about);
}

#[allow(unused)]
pub fn display_version() {
    println!("{}", format!("Novus Package Manager {}", __VERSION__.bright_green().bold()));
}

#[allow(unused)]
pub fn install_help() {
    let install = format!(
        r#"
Novus Package Manager {}

Installs a package or a list of packages

Usage: {} {} {} {}
    
Options:
    
  {} {} Installs a portable version of the package if it exists.
  {} {} Disables colored output for installation.
  {} {} Disables progress bar for installation.
  {} {} Enables multithreaded download for faster installation."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "install".bright_purple(),
        "[package]".white(),
        "[flags]".white(),
        "--portable".bright_cyan(),
        "(-p)".yellow(),
        "--no-color".bright_cyan(),
        "(-nc)".yellow(),
        "--no-progress".bright_cyan(),
        "(-np)".yellow(),
        "--multithreaded".bright_cyan(),
        "(-m)".yellow(),
    );
    println!("{}", install);
}

#[allow(unused)]
pub fn uninstall_help() {
    let uninstall = format!(
        r#"
Novus Package Manager {}
    
Uninstall a package or a list of packages

Usage: {} {} {} {}
    
Options: 
    
  {} {} Disables colored output for uninstallation.  
  {} {} Uninstalls a portable version of the package if it exists."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "uninstall".bright_purple(),
        "[package]".white(),
        "[flags]".white(),
        "--no-color".bright_cyan(),
        "(-nc)".yellow(),
        "--portable".bright_cyan(),
        "(-p)".yellow(),
    );
    println!("{}", uninstall);
}

#[allow(unused)]
pub fn update_help() {
    let update = format!(
        r#"
Novus Package Manager {}

Updates a package or a list of packages

Usage: {} {} {} {}

Options: 
    
  {} {} Disables colored output while updating.
  {} {} Updates a portable version of the package if it exists.
  {} {} Disables progress bar while updating."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "update".bright_purple(),
        "[package]".white(),
        "[flags]".white(),
        "--no-color".bright_cyan(),
        "(-nc)".yellow(),
        "--portable".bright_cyan(),
        "(-p)".yellow(),
        "--no-progress".bright_cyan(),
        "(-np)".yellow(),
    );
    println!("{}", update);
}

#[allow(unused)]
pub fn list_help() {
    let list = format!(
        r#"
Novus Package Manager {}

List all packages available.

Usage: {} {} [number]

Options: 
  {} {} Displays all packages available.
  {} {} Displays package version along with the names.
  {} {} Displays only installed packages.
  {} {} Displays only local packages."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "list".bright_purple(),
        "--all".bright_cyan(),
        "(-a)".yellow(),
        "--version".bright_cyan(),
        "(-v)".yellow(),
        "--installed".bright_cyan(),
        "(-i)".yellow(),
        "--local".bright_cyan(),
        "(-l)".yellow(),
    );
    println!("{}", list);
}

#[allow(unused)]
pub fn info_help() {
    let info = format!(
        r#"
Novus Package Manager {}

Provides information on a specific package.

Usage: {} {} {}

Options: 
  {} {} Displays info only local packages"#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "info".bright_purple(),
        "[package]",
        "--local".bright_cyan(),
        "(-l)".yellow(),
    );
    println!("{}", info);
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
        __VERSION__.bright_green().bold(),
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
}

#[allow(unused)]
pub fn search_help() {
    let search = format!(
        r#"
Novus Package Manager {}

Searches for a package based on input.

Usage: {} {} {}"#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "search".bright_purple(),
        "[query]".white(),
    );
    println!("{}", search);
}

#[allow(unused)]
pub fn quit_help() {
    let quit = format!(
        r#"
Novus Package Manager {}

Quits an application or a list of applications.

Usage: {} {} {}

Options:
  {} {} Accepts all prompts while quiting.
  {} {} Force quits the application."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "quit".bright_purple(),
        "[package]".white(),
        "--yes".bright_cyan(),
        "(-y)".yellow(),
        "--force".bright_cyan(),
        "(-y)".yellow(),
    );
    println!("{}", quit);
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
        __VERSION__.bright_green().bold(),
        "novus quit -f".bright_green(),
        "novus".bright_green(),
        "forcequit".bright_purple(),
        "[package]".white(),
        "--yes".bright_cyan(),
        "(-y)".yellow(),
    );
    println!("{}", forcequit);
}

#[allow(unused)]
pub fn clean_help() {
    let clean = format!(
        r#"
Novus Package Manager {}

Clears novus cache.

Usage: {} {}"#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "clean".bright_purple(),
    );
    println!("{}", clean);
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
}

#[allow(unused)]
pub fn update_error() {
    let update_error = format!(
        r#"
Novus Package Manager {}

{} Missing list of packages to update.
    
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus update --help".bright_green()
    );
    println!("{}", update_error);
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
}

#[allow(unused)]
pub fn list_error(arg: &String) {
    let list_error = format!(
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
    println!("{}", list_error);
}

#[allow(unused)]
pub fn list_number_error() {
    let list_error = format!(
        r#"
Novus Package Manager {}

{} Expected a number. Found a string instead.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus list --help".bright_green()
    );
    println!("{}", list_error);
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
}
