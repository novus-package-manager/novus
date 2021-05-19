use colored::Colorize;

const __VERSION__: &str = "v1.0.0";

pub fn about() {
    let about: String = format!(
        r#"
Novus Package Manager {}

Usage: {} {} [<options>]

Commands:

  {} {} {} Installs packages.
  {} {} {} Uninstalls packages.
  {} {} Updates packages.
  {} {} {} Lists all packages.
  
Run {} for more info about each command."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "[command]".white(),
        "*".bright_magenta().bold(),
        "install".bright_blue(),
        "(i)".yellow(),
        "*".bright_magenta().bold(),
        "uninstall".bright_blue(),
        "(u)".yellow(),
        "*".bright_magenta().bold(),
        "update".bright_blue(),
        "*".bright_magenta().bold(),
        "list".bright_blue(),
        "(search)".yellow(),
        "novus [command] --help".bright_green(),
    );

    println!("{}", about);
    std::process::exit(0);
}

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
        "--no-color".bright_blue(),
        "(-nc)".yellow(),
        "--no-progress".bright_blue(),
        "(-np)".yellow(),
        "--verbose".bright_blue(),
        "(-v)".yellow(),
    );
    println!("{}", init);
    std::process::exit(0);
}

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
        "--no-progress".bright_blue(),
        "(-np)".yellow(),
        "--verbose".bright_blue(),
        "(-v)".yellow()
    );
    println!("{}", install);
    std::process::exit(0);
}

pub fn update_help() {
    let add = format!(
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
        "--no-progress".bright_blue(),
        "(-np)".yellow(),
        "--verbose".bright_blue(),
        "(-v)".yellow(),
    );
    println!("{}", add);
    std::process::exit(0);
}

pub fn list_help() {
    let remove = format!(
        r#"
Novus Package Manager {}

List all packages available.

Usage: {} {}"#,
        format!("novus {}", __VERSION__.bright_green().bold()),
        "novus".bright_green(),
        "list".bright_purple(),
    );
    println!("{}", remove);
    std::process::exit(0);
}

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

pub fn list_packages(packages: Vec<&str>) {    
    println!("novus {} \n\nPackages:\n", __VERSION__.bright_green().bold());
    for package in packages {
        println!("  {}{}", " - ".bright_purple(), package.bright_blue())
    }
    println!("\nRun {} for more info about each command.", "novus [command] --help".bright_green());
    
    std::process::exit(0);
}
