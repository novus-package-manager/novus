use crate::classes::package::Package;
use crate::utils::get_package;
use crate::utils::handle_error::handle_error_and_exit;
use colored::Colorize;
use get_package::get_package;
use indicatif::{ProgressBar, ProgressStyle};
use std::vec;

pub async fn uninstaller(packages: Vec<String>) {
    let mut handles = vec![];
    let mut sizes = vec![];
    let mut multi = false;
    for pkg in packages.iter() {
        let package: Package = get_package(pkg.as_str()).await;
        sizes.push(package.versions[&package.latest_version].size);
    }
    let mut max_size = sizes[0];
    for i in 0..sizes.len() {
        if sizes[i] > max_size {
            max_size = sizes[i];
        }
    }
    if sizes.len() > 1 {
        multi = true;
        println!("{}", "Uninstalling Packages".bright_green());
    }
    for pkg in packages.iter() {
        let pkg_clone = pkg.clone();
        let package: Package = get_package(pkg_clone.as_str()).await;
        let display_name = package.display_name;
        let uswitch = package.uswitches.clone();
        if multi == false {
            println!(
                "{} {}",
                "Uninstalling".bright_green(),
                display_name.bright_green()
            );
        }
        handles.push(std::thread::spawn(move || {
            uninstall(display_name, uswitch);
        }));
    }
    for handle in handles {
        handle
            .join()
            .unwrap_or_else(|_| handle_error_and_exit("An error occured!".to_string()));
    }
    println!("{}", "Successfully uninstalled packages".bright_magenta());
}

#[allow(unused_assignments)]
pub fn uninstall(display_name: String, uswitches: Vec<String>) {
    let mut uninstall_string = get_unins_string(display_name.clone());

    uninstall_string = uninstall_string.clone();
    let split: Vec<&str> = uninstall_string.split(".exe").collect();
    let mut args: Vec<&str> = vec![];
    let mut splits = split[1].replace("\"", "");
    if split[1].contains("/I") {
        splits = splits.replace("/I", "/x");
    }
    for arg in splits.split(" ") {
        if arg != "" {
            args.push(arg);
        }
    }
    for switch in uswitches.iter() {
        if switch != "" {
            args.push(switch.as_str());
        }
    }
    uninstall_string = (split[0].to_string() + ".exe\"").replace("\"", "");
    // println!("args: {:?}", args);
    // println!("uninstall_string: {}", uninstall_string);

    let progress_bar = ProgressBar::new(9999999);
    let pb = progress_bar.clone();

    std::thread::spawn(move || {
        let mut text = String::new();
        // if multi {
        //     text = format!("{}", "Uninstalling Packages".bright_cyan());
        // } else {
        text = format!(
            "{}{}",
            "Uninstalling ".bright_cyan(),
            display_name.bright_cyan()
        );
        // }

        progress_bar.clone().set_style(
            ProgressStyle::default_spinner()
                .template(("{spinner:.green}".to_string() + format!(" {}", text).as_str()).as_str())
                .tick_chars("-\\|/"),
        );
        loop {
            progress_bar.inc(5);
            std::thread::sleep(std::time::Duration::from_millis(100))
        }
    });
    let _cmd = std::process::Command::new(uninstall_string)
        .args(args)
        .spawn()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
        .wait_with_output()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    pb.finish_and_clear();
}

pub fn get_unins_string(display_name: String) -> String {
    use winreg::enums::*;
    use winreg::RegKey;
    // println!("display_name: {}", display_name);
    let mut regkey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut uninstall_string: String = "NULL".to_string();
    for i in 0..2 {
        if i == 1 {
            regkey = RegKey::predef(HKEY_CURRENT_USER);
        }
        let path: RegKey = regkey
            .open_subkey_with_flags(
                "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
                KEY_READ,
            )
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        for name in path
            .enum_keys()
            .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
        {
            let unins_path: RegKey = regkey
                .open_subkey(format!(
                    "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}",
                    name
                ))
                .unwrap_or(
                    regkey
                        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
                        .unwrap_or_else(|e| handle_error_and_exit(e.to_string())),
                );
            let app_name: String = unins_path
                .get_value("DisplayName")
                .unwrap_or("NULL".to_string());
            // println!("app name: {}", app_name);
            if app_name
                .to_lowercase()
                .starts_with(display_name.to_lowercase().as_str())
            {
                uninstall_string = unins_path
                    .get_value("UninstallString")
                    .unwrap_or("NO_STRING".to_string());
            }
        }
    }
    regkey = RegKey::predef(HKEY_LOCAL_MACHINE);
    if uninstall_string == "NULL".to_string() {
        let path: RegKey = regkey.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Installer\\UserData\\S-1-5-18\\Products").unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        for name in path
            .enum_keys()
            .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
        {
            let unins_path: RegKey = regkey.open_subkey(format!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Installer\\UserData\\S-1-5-18\\Products\\{}\\InstallProperties", name)).unwrap_or(regkey.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall").unwrap_or_else(|e| handle_error_and_exit(e.to_string())));
            let app_name: String = unins_path
                .get_value("DisplayName")
                .unwrap_or("NULL".to_string());
            // println!("app name 2: {}", app_name);
            if app_name
                .to_lowercase()
                .starts_with(display_name.to_lowercase().as_str())
            {
                uninstall_string = unins_path
                    .get_value("UninstallString")
                    .unwrap_or("NO_STRING".to_string());
            }
        }
    }

    if uninstall_string == "NULL" {
        handle_error_and_exit(format!("Failed to uninstall {}", display_name));
    }

    uninstall_string.replace("\\", "/")
}
