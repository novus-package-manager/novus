use colored::Colorize;
use std::{io::prelude::*, process};

pub fn quit(apps: Vec<String>, flags: Vec<String>, mut force: bool) {
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
                    exit_code = forcequit_app(app);
                } else {
                    println!("\n{}", "Aborted!".bright_blue());
                    process::exit(0);
                }
            } else {
                exit_code = forcequit_app(app);
            }
        } else {
            exit_code = quit_app(app);
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

fn forcequit_app(app: String) -> i32 {
    let executable = app.to_string() + ".exe";
    let exit_code = process::Command::new("taskkill")
        .args(&["/im", &executable, "/f"])
        .spawn()
        .expect("Failed to terminate process")
        .wait()
        .expect("Failed to terminate process")
        .code()
        .expect("Failed to terminate process");

    exit_code
}

fn quit_app(app: String) -> i32 {
    let executable = app.to_string() + ".exe";
    let exit_code = process::Command::new("taskkill")
        .args(&["/im", &executable])
        .spawn()
        .expect("Failed to terminate process")
        .wait()
        .expect("Failed to terminate process")
        .code()
        .expect("Failed to terminate process");

    exit_code
}
