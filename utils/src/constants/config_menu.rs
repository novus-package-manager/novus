use crate::constants::version::__VERSION__;
use colored::Colorize;

#[allow(unused)]
pub fn config_multithreaded_help() {
    let config_help = format!(
        r#"
Novus Package Manager {}

Changes default threads during downloads.

Usage: {} {} {} {}

Options: 
  {} {} Uses multithreaded downloads during installations.
  {} {} Disables multithreaded downloads during installations."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "config".bright_purple(),
        "multithreaded".white(),
        "[true/false]".white(),
        "true".bright_cyan(),
        "(yes)".yellow(),
        "false".bright_cyan(),
        "(no)".yellow(),
    );
    println!("{}", config_help);
}

#[allow(unused)]
pub fn config_no_color_help() {
    let config_help = format!(
        r#"
Novus Package Manager {}

Changes default colored output during installation.

Usage: {} {} {} {}

Options: 
  {} {} Enables colored output during installations.
  {} {} Disables colored output during installations."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "config".bright_purple(),
        "no-color".white(),
        "[true/false]".white(),
        "true".bright_cyan(),
        "(yes)".yellow(),
        "false".bright_cyan(),
        "(no)".yellow(),
    );
    println!("{}", config_help);
}

#[allow(unused)]
pub fn config_no_progress_help() {
    let config_help = format!(
        r#"
Novus Package Manager {}

Changes default progress indicator during installation.

Usage: {} {} {} {}

Options: 
  {} {} Enables progress indicator during installations.
  {} {} Disables progress indicator during installations."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "config".bright_purple(),
        "no-progress".white(),
        "[true/false]".white(),
        "true".bright_cyan(),
        "(yes)".yellow(),
        "false".bright_cyan(),
        "(no)".yellow(),
    );
    println!("{}", config_help);
}

#[allow(unused)]
pub fn config_portable_help() {
    let config_help = format!(
        r#"
Novus Package Manager {}

Changes default installation method.

Usage: {} {} {} {}

Options: 
  {} {} Always installs portable versions of the package.
  {} {} Does not install portable versions of the package."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "config".bright_purple(),
        "portable".white(),
        "[true/false]".white(),
        "true".bright_cyan(),
        "(yes)".yellow(),
        "false".bright_cyan(),
        "(no)".yellow(),
    );
    println!("{}", config_help);
}

#[allow(unused)]
pub fn config_confirm_help() {
    let config_help = format!(
        r#"
Novus Package Manager {}

Does not prompt for confirmation.

Usage: {} {} {} {}

Options: 
  {} {} Always installs portable versions of the package.
  {} {} Does not install portable versions of the package."#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "config".bright_purple(),
        "confirm".white(),
        "[true/false]".white(),
        "true".bright_cyan(),
        "(yes)".yellow(),
        "false".bright_cyan(),
        "(no)".yellow(),
    );
    println!("{}", config_help);
}

#[allow(unused)]
pub fn config_reset_help() {
    let config_help = format!(
        r#"
Novus Package Manager {}

Resets configuration to default parameters.

Usage: {} {} {}

Alias: {}"#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "config".bright_purple(),
        "reset".white(),
        "default".bright_cyan(),
    );
    println!("{}", config_help);
}

#[allow(unused)]
pub fn config_list_help() {
    let config_help = format!(
        r#"
Novus Package Manager {}

Lists the current configuration.

Usage: {} {} {}"#,
        __VERSION__.bright_green().bold(),
        "novus".bright_green(),
        "config".bright_purple(),
        "list".white(),
    );
    println!("{}", config_help);
}

#[allow(unused)]
pub fn config_error_value() {
    let config_error = format!(
        r#"
Novus Package Manager {}

{} Unknown value specified.
    
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus config --help".bright_green()
    );
    println!("{}", config_error);
}

#[allow(unused)]
pub fn config_error_flag() {
    let config_error = format!(
        r#"
Novus Package Manager {}

{} Unkown flag specified.
    
{} Use {} for more information about this command."#,
        __VERSION__.bright_green().bold(),
        "error".bright_red(),
        "info".bright_blue(),
        "novus config [flag] --help".bright_green()
    );
    println!("{}", config_error);
}