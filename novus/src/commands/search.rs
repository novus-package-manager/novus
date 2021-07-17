use colored::Colorize;
use difflib::get_close_matches;
use utils::classes::{installed_packages::Packages, package::Package};
use utils::constants::version::__VERSION__;
use utils::get_package::get_package;
use utils::handle_error::handle_error_and_exit;

pub async fn search(packages: Vec<&str>, flags: Vec<String>, package: &str) {
    let mut installed = false;
    let mut starts_with = false;
    let mut exact = false;
    let mut version = false;
    if flags.contains(&"--installed".to_string()) || flags.contains(&"-i".to_string()) {
        installed = true;
    }
    if flags.contains(&"--starts-with".to_string()) || flags.contains(&"-sw".to_string()) {
        starts_with = true;
    }
    if flags.contains(&"--exact".to_string()) || flags.contains(&"-e".to_string()) {
        exact = true;
    }
    if flags.contains(&"--version".to_string()) || flags.contains(&"-v".to_string()) {
        version = true;
    }
    println!("Novus Package Manager {} \n", __VERSION__.bright_green().bold());

    #[allow(unused_assignments)]
    let mut installed_packages: Vec<String> = vec![];
    let mut revised_packages: Vec<&str> = vec![];

    if installed {
        let appdata = std::env::var("APPDATA").unwrap();
        let loc = format!(r"{}\novus\config\installed.json", appdata);
        let path = std::path::Path::new(loc.as_str());
        let contents =
            std::fs::read_to_string(path).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let json: Packages = serde_json::from_str::<Packages>(contents.as_str())
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        installed_packages = json.clone().packages;
        if exact {
            for pkg in installed_packages.iter() {
                let pkg_split: Vec<&str> = pkg.split("@").collect();
                let pkg = pkg_split[0];
                if pkg == package {
                    let pkg_split: Vec<&str> = pkg.split("@").collect();
                    revised_packages.push(pkg_split[0]);
                }
            }
        } else if starts_with {
            for pkg in installed_packages.iter() {
                let pkg_split: Vec<&str> = pkg.split("@").collect();
                let pkg = pkg_split[0];
                if pkg.starts_with(package) {
                    let pkg_split: Vec<&str> = pkg.split("@").collect();
                    revised_packages.push(pkg_split[0]);
                }
            }
        } else {
            let new_packages = get_close_matches(
                package,
                installed_packages
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>(),
                10,
                0.4,
            );
            for pkg in new_packages.clone() {
                let pkg_split: Vec<&str> = pkg.split("@").collect();
                revised_packages.push(pkg_split[0]);
            }
        }
    } else {
        if exact {
            for pkg in packages.clone() {
                if pkg == package {
                    revised_packages.push(pkg);
                }
            }
        } else if starts_with {
            for pkg in packages.clone() {
                if pkg.starts_with(&package) {
                    revised_packages.push(pkg);
                }
            }
        } else {
            revised_packages = get_close_matches(package, packages.clone(), 10, 0.4);
        }
    }

    if revised_packages.len() == 0 {
        println!("{}", "No Packages Found!".bright_cyan());
    } else {
        println!("{}", "Closest Matches:\n".bright_purple());
        for package in revised_packages.clone() {
            let pkg: &str = package;
            if version {
                let package_struct: Package = get_package(pkg).await;
                print!("{}{:<60}", " - ".bright_blue(), pkg.bright_cyan());
                println!("{}", package_struct.latest_version.white());
            } else {
                println!("  {}{}", " - ".bright_blue(), pkg.bright_cyan());
            }
        }
    }

    println!(
        "\nRun {} for more info about this command.",
        "novus search --help".bright_green()
    );

    std::process::exit(0);
}
