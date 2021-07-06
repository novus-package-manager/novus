use utils::classes::package::Package;
use utils::autoelevate::autoelevateuninstall;
use utils::handle_error::handle_error_and_exit;
use colored::Colorize;
use utils::get_package::get_package;
use indicatif::{ProgressBar, ProgressStyle};
use std::vec;
use utils::classes::installed_packages::Packages;
use std::process;

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
        let package_name = package.package_name;
        let uswitch = package.uswitches.clone();
        if multi == false {
            println!(
                "{} {}",
                "Uninstalling".bright_green(),
                display_name.bright_green()
            );
        }
        handles.push(std::thread::spawn(move || {
            uninstall(display_name, uswitch, package_name);
        }));
    }
    for handle in handles {
        handle
            .join()
            .unwrap_or_else(|_| handle_error_and_exit("An error occured!".to_string()));
    }
    let temp = std::env::var("TEMP").unwrap();
    let loc = format!(r"{}\novus\config\installed.json", temp);
    let path = std::path::Path::new(loc.as_str());
    if path.exists() {
        let contents = std::fs::read_to_string(path).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let json: Packages = serde_json::from_str::<Packages>(contents.as_str()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let mut installed_packages = json.clone().packages;
        for package in packages {
            for installed_package in installed_packages.clone() {
                if installed_package.starts_with(&package) {
                    let index = installed_packages.iter().position(|x| *x == installed_package).unwrap();
                    installed_packages.remove(index);                  
                }
            }
        }
        let installed_packages: Packages = Packages {
            packages: installed_packages
        };
        let file = std::fs::File::create(path).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        serde_json::to_writer_pretty(file, &installed_packages).unwrap();
    }
    println!("{}", "Successfully uninstalled packages".bright_magenta());
}

#[allow(unused_assignments)]
pub fn uninstall(display_name: String, uswitches: Vec<String>, package_name: String) {
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

    let output;

    if uninstall_string.starts_with("MsiExec.exe") {
        let mut msi_args = args.clone();
        msi_args.push("/passive");
        output = std::process::Command::new(uninstall_string.clone())
        .args(msi_args).output();
    }
    else {
        output = std::process::Command::new(uninstall_string.clone())
        .args(args.clone()).output();
    }

    let mut code = 0;

    let output = output.unwrap_or_else(|e| {
        if e.to_string().contains("requires elevation") {            
            args.insert(0, &package_name);
            if uninstall_string.starts_with("MsiExec.exe") {
                let mut msi_args = args.clone();
                msi_args.push("/passive");
                code = autoelevateuninstall(msi_args);
            }
            else {
                code = autoelevateuninstall(args);
            }
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
        let error_message = String::from_utf8(output.stderr)
            .unwrap_or("Failed to uninstall packages".to_string());
            pb.finish_and_clear();
            println!("{}", error_message.bright_red());
            process::exit(0);
    } else {
        pb.finish_and_clear();
        println!("{}", "Successfully Uninstalled Packages".bright_purple());
        process::exit(0);
    }
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
                    .unwrap_or("NULL".to_string());
            }
        }
    }

    if uninstall_string == "NULL".to_string() {
        let path: RegKey = regkey.open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall").unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        for name in path
            .enum_keys()
            .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
        {
            let unins_path: RegKey = regkey.open_subkey(format!("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}", name)).unwrap_or(regkey.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall").unwrap_or_else(|e| handle_error_and_exit(e.to_string())));
            let app_name: String = unins_path
                .get_value("DisplayName")
                .unwrap_or("NULL".to_string());
            // println!("app name 3: {}", app_name);
            if app_name
                .to_lowercase()
                .starts_with(display_name.to_lowercase().as_str())
            {
                uninstall_string = unins_path
                    .get_value("UninstallString")
                    .unwrap_or("NULL".to_string());
            }
        }
    }

    if uninstall_string == "NULL" {
        handle_error_and_exit(format!("Failed to uninstall {}", display_name));
    }

    uninstall_string.replace("\\", "/")
}