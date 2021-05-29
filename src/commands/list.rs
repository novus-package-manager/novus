use crate::classes::{installed_packages::Packages, package::Package};
use crate::constants::version::__VERSION__;
use crate::utils::get_package::get_package;
use crate::utils::handle_error::handle_error_and_exit;
use colored::Colorize;

pub async fn list(packages: Vec<&str>, flags: Vec<String>) {
    let mut installed = false;
    let mut version = false;
    if flags.contains(&"--installed".to_string()) || flags.contains(&"-i".to_string()) {
        installed = true;
    }
    if flags.contains(&"--version".to_string()) || flags.contains(&"-v".to_string()) {
        version = true;
    }
    println!(
        "novus {} \n\n{}\n",
        __VERSION__.bright_green().bold(),
        "Packages:".bright_purple(),
    );
    if installed {
        let temp = std::env::var("TEMP").unwrap();
        let loc = format!(r"{}\novus\config\installed.json", temp);
        let path = std::path::Path::new(loc.as_str());
        let contents =
            std::fs::read_to_string(path).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let json: Packages = serde_json::from_str::<Packages>(contents.as_str())
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let installed_packages = json.clone().packages;
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
        for package in packages {
            if version {
                let package_struct: Package = get_package(package).await;
                print!("{}{:<60}", " - ".bright_blue(), package.bright_cyan());
                println!("{}", package_struct.latest_version.white());
            } else {
                println!("  {}{}", " - ".bright_blue(), package.bright_cyan())
            }
        }
    }
    println!(
        "\nRun {} for more info about each command.",
        "novus [command] --help".bright_green()
    );
    std::process::exit(0);
}
