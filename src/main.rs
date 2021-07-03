#![cfg(windows)]

mod classes;
mod commands;
mod constants;
mod utils;

use colored::Colorize;
use commands::{clean, info, install, list, quit, search, uninstall};
use display_help::display_help;

use clean::clean;
use constants::commands::COMMANDS;
use handle_args::{get_arguments, verify_args};
use info::info;
use install::installer;
use list::list;
use quit::quit;
use search::search;
use serde_json::Value;
use uninstall::uninstaller;
use utils::{display_help, get_package, handle_args, handle_error::handle_error_and_exit};
// use std::time::Instant;

#[allow(unused)]
#[tokio::main]
async fn main() {
    let _enabled = ansi_term::enable_ansi_support();

    create_dirs();

    ctrlc::set_handler(move || {
        println!("\n{}", "Aborted!".bright_blue());
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let args: Vec<String> = std::env::args().collect();

    let command = display_help(&args);

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

    let mut command: &str = command;

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

    match command {
        "install" => {
            installer(packages, flags).await;
        }
        "uninstall" => {
            uninstaller(packages).await;
        }
        "update" => {
            installer(packages, flags).await;
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
        &_ => {}
    }
}

fn create_dirs() {
    let temp = std::env::var("TEMP").unwrap();
    let loc = format!(r"{}\novus\", temp);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }
    let loc = format!(r"{}\novus\config", temp);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }
    let loc = format!(r"{}\novus\scripts", temp);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        let _ = std::fs::create_dir(path);
    }
    let loc = format!(r"{}\novus\scripts\auto_elevate_install.bat", temp);
    let path = std::path::Path::new(loc.as_str());
    if !path.exists() {
        std::fs::write(path, "@echo off

        :: BatchGotAdmin
        :-------------------------------------
        REM  --> Check for permissions
            IF \"%PROCESSOR_ARCHITECTURE%\" EQU \"amd64\" (
        >nul 2>&1 \"%SYSTEMROOT%\\SysWOW64\\cacls.exe\" \"%SYSTEMROOT%\\SysWOW64\\config\\system\"
        ) ELSE (
        >nul 2>&1 \"%SYSTEMROOT%\\system32\\cacls.exe\" \"%SYSTEMROOT%\\system32\\config\\system\"
        )
        
        REM --> If error flag set, we do not have admin.
        if '%errorlevel%' NEQ '0' (
            goto UACPrompt
        ) else ( goto gotAdmin )
        
        :UACPrompt
            echo Set UAC = CreateObject^(\"Shell.Application\"^) > \"%temp%\\getadmin.vbs\"
            set params= %*
            echo UAC.ShellExecute \"cmd.exe\", \"/c \"\"%~s0\"\" %params:\"=\"\"%\", \"\", \"runas\", 1 >> \"%temp%\\getadmin.vbs\"
        
            \"%temp%\\getadmin.vbs\"
            del \"%temp%\\getadmin.vbs\"
            exit /B
        
        :gotAdmin
            pushd \"%CD%\"
            CD /D \"%~dp0\"
        :--------------------------------------
        
        powershell novus i %1
        timeout /t 2").unwrap_or_else(|_| handle_error_and_exit("Failed to write bat file".to_string()));
    }
}
