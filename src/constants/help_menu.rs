use colored::Colorize;

const __VERSION__: &str = "v1.0.0";

pub fn about() {
    let about: String = format!(
        r#"
Novus Package Manager {}

Usage: {} {} [<options>]

Commands:

  {} {} - Installs packages.
  {} {} - Uninstalls packages.
  {} {} - Updates packages.
  {} {} - Lists all packages."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green().bold(),
        "[command]".white(),
        "*".bright_magenta().bold(),
        "install".bright_blue(),
        "*".bright_magenta().bold(),
        "uninstall".bright_blue(),
        "*".bright_magenta().bold(),
        "update".bright_blue(),
        "*".bright_magenta().bold(),
        "list".bright_blue()
    );

    println!("{}", about);
    std::process::exit(0);
}

pub fn install_help() {
    let init = format!(
        r#"
Novus Package Manager {}

Installs a package

Usage: {} {} {} {}
    
Options:
    
  {} {} Disable colored output for installation.  
  {} {} Output verbose messages on internal operations."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green().bold(),
        "install".bright_purple(),
        "[package]".white(),
        "[flags]".white(),
        "--no-color".blue(),
        "(-nc)".yellow(),
        "--verbose".blue(),
        "(-v)".yellow()
    );
    println!("{}", init);
    std::process::exit(0);
}

pub fn uninstall_help() {
    let install = format!(
        r#"{}
    
Install dependencies for a project.

Usage: {} {} {}
    
Options: 
    
  {} {} Accept all prompts while installing dependencies.  
  {} {} Output verbose messages on internal operations."#,
        format!("volt {}", __VERSION__.bright_green().bold()),
        "volt".bright_green().bold(),
        "install".bright_purple(),
        "[flags]".white(),
        "--yes".blue(),
        "(-y)".yellow(),
        "--verbose".blue(),
        "(-v)".yellow()
    );
    println!("{}", install);
    std::process::exit(0);
}

pub fn update_help() {
    let add = format!(
        r#"{}

Add a package to your dependencies for your project.

Usage: {} {} {} {}

Options: 
    
  {} {} Output the version number.
  {} {} Output verbose messages on internal operations.
  {} {} Disable progress bar."#,
        format!("volt {}", __VERSION__.bright_green().bold()),
        "volt".bright_green().bold(),
        "add".bright_purple(),
        "[packages]".white(),
        "[flags]".white(),
        "--version".blue(),
        "(-ver)".yellow(),
        "--verbose".blue(),
        "(-v)".yellow(),
        "--no-progress".blue(),
        "(-np)".yellow()
    );
    println!("{}", add);
    std::process::exit(0);
}

pub fn list_help() {
    let remove = format!(
        r#"{}

Removes a package from your direct dependencies.

Usage: {} {} {} {}

Options: 

  {} {} Output the version number.
  {} {} Output verbose messages on internal operations."#,
        format!("volt {}", __VERSION__.bright_green().bold()),
        "volt".bright_green().bold(),
        "remove".bright_purple(),
        "[packages]".white(),
        "[flags]".white(),
        "--version".blue(),
        "(-ver)".yellow(),
        "--verbose".blue(),
        "(-v)".yellow()
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
    std::process::exit(1);
}

pub fn uninstall_error() {
    let uninstall_error = format!(
        r#"
Novus Package Manager {}

{} Missing list of packages to install.
      
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus install --help".bright_green()
    );
    println!("{}", uninstall_error);
    std::process::exit(1);
}

pub fn invalid_command(command: &str) {
    println!(
        "{} {}\n{} Use {} for the list of all the commands\n",
        "error".bright_red(),
        format!("{} is not a valid command!", command.blue()),
        "info".bright_blue(),
        "novus --help".bright_green()
    );
    std::process::exit(1);
}
