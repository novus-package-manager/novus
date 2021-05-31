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
  {} {} Displays only the package names"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "list".bright_purple(),
        "--all".bright_cyan(),
        "(-a)".yellow(),
        "--installed".bright_cyan(),
        "(-i)".yellow(),
        "--names".bright_cyan(),
        "(-n)".yellow(),
    );
    println!("{}", list);
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
    let uninstall_error = format!(
        r#"
Novus Package Manager {}

{} Missing keyword to search for.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus search --help".bright_green()
    );
    println!("{}", uninstall_error);
    std::process::exit(0);
}

#[allow(unused)]
pub fn quit_error() {
    let uninstall_error = format!(
        r#"
Novus Package Manager {}

{} Missing application to quit.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus quit --help".bright_green()
    );
    println!("{}", uninstall_error);
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
