use std::path::Path;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::process;
use std::vec;
use std::fs;
use utils::autoelevate::autoelevateuninstall;
use utils::classes::package::Package;
use utils::get_package::get_package;
use utils::handle_error::handle_error_and_exit;
use utils::registry::get_unins_string;
use utils::classes::config::Config;

pub async fn uninstaller(inital_packages: Vec<String>, flags: Vec<String>, package_list: Vec<&str>, config: Config) -> i32 {
    let mut no_color = config.no_color;
    let mut portable_flag = config.portable;
    if flags.contains(&"--no-color".to_string()) || flags.contains(&"-nc".to_string()) {
        no_color = true;
    }
    if flags.contains(&"--portable".to_string()) || flags.contains(&"-p".to_string()) {
        portable_flag = true;
    }

    let mut packages: Vec<String> = inital_packages.clone();

    if portable_flag {
        for pkg in inital_packages {
            let pkg_portable = pkg.clone() + "-portable";
            let index = packages.iter().position(|x| *x == pkg.clone()).unwrap();
            packages.remove(index);
            if package_list.contains(&pkg_portable.as_str()) {
                packages.push(pkg_portable)
            }
            else {
                println!("{} {}", "Couldn't find a portable package for".bright_red(), pkg.bright_red());
                if packages.len() == 0 {
                    process::exit(1);
                }
            }
        }
    }

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
        let package_name = package.package_name;
        let exec_name = package.exec_name;
        let portable = package.portable;
        let uswitch = package.uswitches.clone();
        if multi == false {
            println!(
                "{} {}",
                "Uninstalling".bright_green(),
                display_name.bright_green()
            );
        }
        handles.push(std::thread::spawn(move || {
            let code: i32;
            if portable == Some(true) {
                code = uninstall_portable(display_name, package_name, exec_name);
            }
            else {
                code = uninstall(display_name, uswitch, package_name, no_color);
            }
            code
        }));
    }

    let mut codes: Vec<i32> = vec![];

    for handle in handles {
        codes.push(
            handle
                .join()
                .unwrap_or_else(|_| handle_error_and_exit("An error occured!".to_string())),
        );
    }

    if codes.contains(&1) {
        return 1;
    }

    0
}

fn uninstall_portable(display_name: String, package_name: String, exec_name: String) -> i32 {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate appdata directory".to_string())
    });

    let user_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate user profile directory".to_string())
    });

    // Remove Shortcut
    let star_menu_loc = format!(r"{}\Microsoft\Windows\Start Menu\Programs\Novus", appdata);
    let start_menu_dir = Path::new(&star_menu_loc);
    for entry in fs::read_dir(start_menu_dir).unwrap_or_else(|e| handle_error_and_exit(e.to_string())) {
        let entry = entry.unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let path = entry.path();
        let path_str = path.display().to_string();
        if path_str.contains(&display_name) {
            fs::remove_file(path).unwrap_or_else(|e| handle_error_and_exit(format!("Failed to remove shurtcut: {}", e.to_string())));
        }
    }

    // Remove shims
    let shims_dir = Path::new(&user_profile).join("novus").join("shims");
    for entry in fs::read_dir(shims_dir).unwrap_or_else(|e| handle_error_and_exit(e.to_string())) {
        let entry = entry.unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let path = entry.path();
        let path_str = path.display().to_string();
        if path_str.contains(&exec_name) {
            fs::remove_file(path).unwrap_or_else(|e| handle_error_and_exit(format!("Failed to remove shim: {}", e.to_string())));
        }
    }

    // Remove tools
    let tools_dir = Path::new(&user_profile).join("novus").join("tools");
    for entry in fs::read_dir(tools_dir).unwrap_or_else(|e| handle_error_and_exit(e.to_string())) {
        let entry = entry.unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let path = entry.path();
        let path_str = path.display().to_string();
        if path_str.contains(&package_name) {
            fs::remove_dir_all(path).unwrap_or_else(|e| handle_error_and_exit(format!("Failed to remove tool: {}", e.to_string())));
        }
    }

    println!("{}", "Successfully Uninstalled Packages".bright_purple());

    0
}

#[allow(unused_assignments)]
pub fn uninstall(display_name: String, uswitches: Vec<String>, package_name: String, no_color: bool) -> i32 {
    let mut uninstall_string = get_unins_string(display_name.clone());

    uninstall_string = uninstall_string.clone();
    let mut split: Vec<&str> = vec![];
    let mut file_extension: &str = ".exe";
    if uninstall_string.contains(".exe") {
        split = uninstall_string.split(".exe").collect();
    }
    if uninstall_string.contains(".bat") {
        split = uninstall_string.split(".bat").collect();
        file_extension = ".bat";
    }
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
    uninstall_string = (split[0].to_string() + file_extension + "\"").replace("\"", "");

    let progress_bar = ProgressBar::new(0);
    let pb = progress_bar.clone();

    std::thread::spawn(move || {
        let mut text = String::new();

        if no_color {
            text = format!(
                "Uninstalling {}",
                display_name
            );
            progress_bar.clone().set_style(
                ProgressStyle::default_spinner()
                    .template(("{spinner:.white}".to_string() + format!(" {}", text).as_str()).as_str())
                    .tick_chars("-\\|/"),
            );
        }
        else {
            text = format!(
                "{}{}",
                "Uninstalling ".bright_cyan(),
                display_name.bright_cyan()
            );
            progress_bar.clone().set_style(
                ProgressStyle::default_spinner()
                    .template(("{spinner:.green}".to_string() + format!(" {}", text).as_str()).as_str())
                    .tick_chars("-\\|/"),
            );
        }
    
        loop {
            progress_bar.inc(5);
            std::thread::sleep(std::time::Duration::from_millis(100))
        }
    });

    let output;

    if uninstall_string.starts_with("MsiExec.exe") {
        let mut msi_args = args.clone();
        msi_args.push("/passive");
        output = std::process::Command::new(uninstall_string.clone())
            .args(msi_args)
            .output();
    } else {
        output = std::process::Command::new(uninstall_string.clone())
            .args(args.clone())
            .output();
    }

    let mut code = 0;

    let output = output.unwrap_or_else(|e| {
        if e.to_string().contains("requires elevation") {
            code = autoelevateuninstall(package_name);
            pb.finish_and_clear();
            println!("{}", "Auto Elevating".bright_cyan());

            process::exit(0)
        } else {
            handle_error_and_exit(e.to_string());
        }
    });

    code = output
        .status
        .code()
        .unwrap_or_else(|| handle_error_and_exit("Failed to retrieve exit code".to_string()));
    if code == 1 {
        let error_message =
            String::from_utf8(output.stderr).unwrap_or("Failed to uninstall packages".to_string());
        pb.finish_and_clear();
        println!("{}", error_message.bright_red());
        process::exit(0);
    } else {
        pb.finish_and_clear();
        println!("{}", "Successfully Uninstalled Packages".bright_purple());
    }

    code
}