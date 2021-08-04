use colored::Colorize;
use std::{io::prelude::*, process};
use utils::classes::package::Package;
use utils::get_package::get_package;
use utils::handle_error::handle_error_and_exit;

pub async fn quit(apps: Vec<String>, flags: Vec<String>, mut force: bool) {
    let mut confirm = false;
    if flags.contains(&"-f".to_string()) || flags.contains(&"--force".to_string()) {
        force = true;
    }
    if flags.contains(&"-y".to_string()) || flags.contains(&"--yes".to_string()) {
        confirm = true;
    }
    let mut exit_code = 0;
    for app in apps.clone() {
        if force {
            if !confirm {
                print!("Are you sure you want to force quit {} (Y/N): ", app);
                std::io::stdout()
                    .flush()
                    .ok()
                    .expect("Could not flush stdout");
                let mut string: String = String::new();
                let _ = std::io::stdin().read_line(&mut string);
                if string.trim().to_lowercase() == "y" {
                    exit_code = forcequit_app(app).await;
                } else {
                    println!("\n{}", "Aborted!".bright_blue());
                    process::exit(0);
                }
            } else {
                exit_code = forcequit_app(app).await;
            }
        } else {
            exit_code = quit_app(app).await;
        }
    }
    if exit_code == 0 {
        if apps.len() == 1 {
            println!(
                "{} {}",
                "Successfully Terminated".bright_purple(),
                apps[0].bright_purple()
            )
        } else {
            println!("{}", "Successfully Terminated Apps".bright_purple())
        }
    } else {
        process::exit(1);
    }
}

async fn forcequit_app(app: String) -> i32 {
    let package: Package = get_package(&app).await;
    let exec_name = package.exec_name;
    if exec_name == "none" {
        println!(
            "{} {}",
            "Cannot terminate".bright_purple(),
            app.bright_purple()
        );
        process::exit(0);
    }
    let mut executable = exec_name.clone();
    if !exec_name.contains(".") {
        executable = exec_name + ".exe";
    }
    let output = process::Command::new("taskkill")
        .args(&["/im", &executable, "/f"])
        .output()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let msg = std::str::from_utf8(&output.stderr).unwrap();
    if msg.ends_with("not found.\r\n") {
        println!(
            "{}",
            "Cannot terminate a proccess which is not running!".bright_purple()
        );
        process::exit(0);
    }

    0
}

async fn quit_app(app: String) -> i32 {
    let package: Package = get_package(&app).await;
    let exec_name = package.exec_name;
    if exec_name == "none" {
        println!(
            "{} {}",
            "Cannot terminate".bright_purple(),
            app.bright_purple()
        );
        process::exit(0);
    }
    let mut executable = exec_name.clone();
    if !exec_name.contains(".") {
        executable = exec_name + ".exe";
    }
    let output = process::Command::new("taskkill")
        .args(&["/im", &executable])
        .output()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let msg = std::str::from_utf8(&output.stderr).unwrap();
    if msg.ends_with("not found.\r\n") {
        println!(
            "{}",
            "Cannot terminate a proccess which is not running!".bright_purple()
        );
        process::exit(0);
    }
    if msg.ends_with("This process can only be terminated forcefully (with /F option).\r\n") {
        println!(
            "{} {} {} {} {} {}",
            "Failed to terminate process.".bright_purple(),
            "\n\nTry running with the",
            "-f".bright_green(),
            "flag or use",
            "forcequit".bright_green(),
            "instead"
        );
        process::exit(0);
    }

    0
}
