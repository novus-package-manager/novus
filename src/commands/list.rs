use crate::classes::{installed_packages::Packages, package::Package};
use crate::constants::help_menu::{list_error, list_number_error};
use crate::constants::version::__VERSION__;
use crate::utils::get_package::get_package;
use crate::utils::handle_error::handle_error_and_exit;
use crate::utils::registry::get_local_packages;
use colored::Colorize;

pub async fn list(packages: Vec<&str>, flags: Vec<String>, args: Vec<String>) {
    let number = verify_args(args);
    let mut installed = false;
    let mut version = false;
    let mut local = false;
    if flags.contains(&"--installed".to_string()) || flags.contains(&"-i".to_string()) {
        installed = true;
    }
    if flags.contains(&"--version".to_string()) || flags.contains(&"-v".to_string()) {
        version = true;
    }
    if flags.contains(&"--local".to_string()) || flags.contains(&"-l".to_string()) {
        local = true;
    }
    println!(
        "\nNovus Package Manager {}\n",
        __VERSION__.bright_green().bold()
    );
    if installed {
        let temp = std::env::var("TEMP").unwrap();
        let loc = format!(r"{}\novus\config\installed.json", temp);
        let path = std::path::Path::new(loc.as_str());
        let contents =
            std::fs::read_to_string(path).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let json: Packages = serde_json::from_str::<Packages>(contents.as_str())
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let installed_packages: Vec<String> = json.clone().packages;
        if number as usize >= installed_packages.len() || number == 0 {
            if installed_packages.len() != 0 {
                println!("{}\n", "Packages: ".bright_purple());
            } else {
                println!("{}", "No packages found!".bright_cyan());
            }
            for package in installed_packages {
                let package_split: Vec<&str> = package.split("@").collect();
                let pkg: &str = package_split[0];
                if version {
                    let package_struct: Package = get_package(pkg).await;
                    print!("{}{:<60}", " - ".bright_blue(), pkg.bright_cyan());
                    println!("{}", package_struct.latest_version.white());
                } else {
                    println!("  {}{}", " - ".bright_blue(), pkg.bright_cyan())
                }
            }
        } else {
            if installed_packages.len() != 0 {
                println!("{}\n", "Packages: ".bright_purple());
            } else {
                println!("{}", "No packages found!".bright_cyan());
            }
            for index in 0..number {
                let package = &installed_packages[index as usize];
                let package_split: Vec<&str> = package.split("@").collect();
                let pkg: &str = package_split[0];
                if version {
                    let package_struct: Package = get_package(pkg).await;
                    print!("{}{:<60}", " - ".bright_blue(), pkg.bright_cyan());
                    println!("{}", package_struct.latest_version.white());
                } else {
                    println!("  {}{}", " - ".bright_blue(), pkg.bright_cyan())
                }
            }
        }
    } else if local {
        let name_versions = get_local_packages();
        if number as usize >= packages.len() || number == 0 {
            if name_versions.len() != 0 {
                println!("{}\n", "Packages: ".bright_purple());
            } else {
                println!("{}", "No packages found!".bright_cyan());
            }
            for (display_name, display_version) in name_versions {
                if display_name != "Unknown" {
                    if version {
                        print!("{}{:<80}", " - ".bright_blue(), display_name.bright_cyan());
                        println!("{}", display_version);
                    } else {
                        println!("  {}{}", " - ".bright_blue(), display_name.bright_cyan())
                    }
                }
            }
        } else {
            if name_versions.len() != 0 {
                println!("{}\n", "Packages: ".bright_purple());
            } else {
                println!("{}", "No packages found!".bright_cyan());
            }
            for index in 0..number {
                let (display_name, display_version) = &name_versions[index as usize];
                if display_name != "Unknown" {
                    if version {
                        print!("{}{:<80}", " - ".bright_blue(), display_name.bright_cyan());
                        println!("{}", display_version);
                    } else {
                        println!("  {}{}", " - ".bright_blue(), display_name.bright_cyan())
                    }
                }
            }
        }
    } else {
        if number as usize >= packages.len() || number == 0 {
            if packages.len() != 0 {
                println!("{}\n", "Packages: ".bright_purple());
            } else {
                println!("{}", "No packages found!".bright_cyan());
            }
            for package in packages {
                if version {
                    let package_struct: Package = get_package(package).await;
                    print!("{}{:<60}", " - ".bright_blue(), package.bright_cyan());
                    println!("{}", package_struct.latest_version.white());
                } else {
                    println!("  {}{}", " - ".bright_blue(), package.bright_cyan())
                }
            }
        } else {
            if packages.len() != 0 {
                println!("{}\n", "Packages: ".bright_purple());
            } else {
                println!("{}", "No packages found!".bright_cyan());
            }
            for index in 0..number {
                let package = &packages[index as usize];
                if version {
                    let package_struct: Package = get_package(package).await;
                    print!("{}{:<60}", " - ".bright_blue(), package.bright_cyan());
                    println!("{}", package_struct.latest_version.white());
                } else {
                    println!("  {}{}", " - ".bright_blue(), package.bright_cyan())
                }
            }
        }
    }
    println!(
        "\nRun {} for more info about each command.",
        "novus [command] --help".bright_green()
    );
    std::process::exit(0);
}

fn verify_args(arguments: Vec<String>) -> u64 {
    let mut args = vec![];
    for argument in arguments {
        if !argument.starts_with("-") {
            args.push(argument);
        }
    }
    let mut number = 0;
    if args.len() > 3 {
        list_error(&args[3]);
    }
    if args.len() == 3 {
        number = args[2].parse().unwrap_or_else(|_| {
            list_number_error();
            0
        });
    }

    number
}
