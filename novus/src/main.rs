mod commands;
use colored::Colorize;
use commands::{clean, info, install, list, quit, search, uninstall, config};
use display_help::display_help;

use clean::clean;
use handle_args::{get_arguments, verify_args};
use info::info;
use install::installer;
use list::list;
use quit::quit;
use search::search;
use serde_json::Value;
use std::time::Instant;
use uninstall::uninstaller;
use config::config;
use utils::check_version::check_version;
use utils::constants::commands::COMMANDS;
use utils::scripts::auto_elevate_scripts::{AUTO_ELEVATE_INSTALL, AUTO_ELEVATE_UNINSTALL};
use utils::{display_help, get_package, handle_args, handle_error::handle_error_and_exit};

#[allow(unused)]
#[tokio::main]
async fn main() {
    let _enabled = ansi_term::enable_ansi_support();

    // Starts a timer
    let start = Instant::now();

    colored::control::set_override(true);

    create_dirs();

    let update_available: bool = check_version().await;

    ctrlc::set_handler(move || {
        println!("\n{}", "Aborted!".bright_cyan());
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let args: Vec<String> = std::env::args().collect();

    let command: String = display_help(&args).await.to_string();

    #[allow(unused)]
    let mut data = String::new();

    data = get_package::get_packages().await;

    let val = data
        .as_str()
        .parse::<Value>()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let package_list: Vec<&str> = val["packages"]
        .as_array()
        .unwrap_or_else(|| handle_error_and_exit("An error occured".to_string()))
        .iter()
        .map(|p| {
            p.as_str()
                .unwrap_or_else(|| handle_error_and_exit("An error occured".to_string()))
        })
        .collect();

    let mut command: &str = &command;

    for cmd in COMMANDS.iter() {
        if command == cmd[1] {
            command = cmd[0];
        }
    }

    let (flags, packages) = get_arguments(&args);

    let (flags, packages) = verify_args(
        flags.clone(),
        packages.clone(),
        command,
        package_list.clone(),
    );

    if flags.contains(&"--no-color".to_string()) || flags.contains(&"-nc".to_string()) {
        colored::control::set_override(false);
    }

    let mut code = 0;

    match command {
        "install" => {
            installer(packages, package_list, flags, false).await;
        }
        "uninstall" => {
            code = uninstaller(packages, flags, package_list).await;
        }
        "update" => {
            code = installer(packages, package_list, flags, true).await;
        }
        "list" => {
            list(package_list, flags, args).await;
        }
        "clean" => {
            clean(args);
        }
        "search" => {
            search(package_list, flags, &packages[0]).await;
        }
        "quit" => {
            quit(packages, flags, false).await;
        }
        "forcequit" => {
            quit(packages, flags, true).await;
        }
        "info" => {
            info(args, package_list).await;
        }
        "config" => {
            config(args, flags).await;
        }
        &_ => {}
    }

    println!(
        "Completed in {}.{:.*}s",
        start.elapsed().as_secs(),
        2,
        start.elapsed().as_millis().to_string(),
    );

    if update_available {
        println!(
            "\n{} A new version of Novus is available. Run {} to update.",
            "NOTE".bright_cyan(),
            "novus update novus".bright_cyan()
        );
    }

    std::process::exit(0)
}

fn create_dirs() {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate appdata directory".to_string())
    });
    let user_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate user profile directory".to_string())
    });
    let loc = format!(r"{}\novus\", appdata);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }
    let loc = format!(r"{}\novus\", user_profile);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }
    let loc = format!(r"{}\novus\scripts", appdata);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }
    let loc = format!(r"{}\novus\scripts\auto_elevate_install.bat", appdata);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        std::fs::write(path, AUTO_ELEVATE_INSTALL)
            .unwrap_or_else(|_| handle_error_and_exit("Failed to write bat file".to_string()));
    }
    let loc = format!(r"{}\novus\scripts\auto_elevate_uninstall.bat", appdata);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        std::fs::write(path, AUTO_ELEVATE_UNINSTALL)
            .unwrap_or_else(|_| handle_error_and_exit("Failed to write bat file".to_string()));
    }
}
