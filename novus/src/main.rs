mod commands;
use colored::Colorize;
use commands::{alias, clean, info, install, list, quit, search, status, uninstall};
use display_help::display_help;

use alias::alias;
use clean::clean;
use handle_args::{get_arguments, verify_args};
use info::info;
use install::installer;
use list::list;
use quit::quit;
use search::search;
use serde_json::Value;
use status::status;
use std::time::Instant;
use uninstall::uninstaller;
use utils::check_version::check_version;
use utils::classes::config::Config;
use utils::constants::commands::COMMANDS;
use utils::scripts::auto_elevate_scripts::{AUTO_ELEVATE_INSTALL, AUTO_ELEVATE_UNINSTALL};
use utils::{display_help, get_package, handle_args, handle_error::handle_error_and_exit};

#[allow(unused)]
#[tokio::main]
async fn main() {
    ansi_term::enable_ansi_support();

    // Starts a timer
    let start = Instant::now();

    colored::control::set_override(true);

    create_dirs();

    let config: Config = get_config();

    let mut update_available = false;

    let check_updates = tokio::spawn(async move {
        update_available = check_version().await;
    });

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

    let mut command: String = command;

    for cmd in COMMANDS.iter() {
        if command == cmd[1] {
            command = cmd[0].to_string();
        }
    }

    let (flags, packages, install_path) = get_arguments(&args);

    // println!("install path: {}", install_path);

    let (flags, packages) = verify_args(
        flags.clone(),
        packages.clone(),
        &command,
        package_list.clone(),
    );

    let packages_clone = packages.clone();
    let command_clone = command.clone();

    ctrlc::set_handler(move || {
        if command_clone == "update" && packages_clone.contains(&"novus".to_string()) {
            println!("\n{}", "Successfully Updated Novus!".bright_cyan());
            std::process::exit(0);
        } else {
            println!("\n{}", "Aborted!".bright_cyan());
            std::process::exit(0);
        }
    })
    .expect("Error setting Ctrl-C handler");

    if flags.contains(&"--no-color".to_string())
        || flags.contains(&"-nc".to_string())
        || config.no_color
    {
        colored::control::set_override(false);
    }

    let mut code = 0;

    match command.as_str() {
        "install" => {
            installer(packages, package_list, flags, false, config, install_path).await;
        }
        "uninstall" => {
            code = uninstaller(packages, flags, package_list, config).await;
        }
        "update" => {
            code = installer(packages, package_list, flags, true, config, install_path).await;
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
            quit(packages, flags, false, config).await;
        }
        "forcequit" => {
            quit(packages, flags, true, config).await;
        }
        "info" => {
            info(args, package_list).await;
        }
        "config" => {
            commands::config::config(args, flags).await;
        }
        "status" => {
            status(flags, &packages[0]).await;
        }
        "alias" => {
            alias(args, package_list).await;
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
    let loc = format!(r"{}\novus\aliases.json", appdata);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::File::create(path);
        std::fs::write(
            path,
            "
{
    \"aliases\": {}
}
        ",
        )
        .unwrap_or_else(|_| handle_error_and_exit("Failed to write bat file".to_string()));
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

fn get_config() -> Config {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate appdata directory".to_string())
    });
    let loc = format!(r"{}\novus\config", appdata);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }

    let loc = format!(r"{}\novus\config\config.json", appdata);
    let path = std::path::Path::new(loc.as_str());

    if !path.exists() {
        let config_json = Config {
            multithreaded: false,
            no_color: false,
            no_progress: false,
            portable: false,
            confirm: false,
            installpath: "DEFAULT".to_string(),
        };

        let config_file = std::fs::File::create(path)
            .unwrap_or_else(|_| handle_error_and_exit("Failed to create config file".to_string()));

        serde_json::to_writer_pretty(config_file, &config_json)
            .unwrap_or_else(|_| handle_error_and_exit("Failed to write config file".to_string()));

        return config_json;
    } else {
        let contents = std::fs::read_to_string(path)
            .unwrap_or_else(|_| handle_error_and_exit("Failed to open config file".to_string()));
        let config: Config = serde_json::from_str::<Config>(&contents)
            .unwrap_or_else(|_| handle_error_and_exit("Failed to parse config file".to_string()));
        return config;
    }
}
