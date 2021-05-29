use crate::constants::version::__VERSION__;
use colored::Colorize;
use crate::classes::installed_packages::Packages;
use crate::utils::handle_error::handle_error_and_exit;

pub fn list(packages: Vec<&str>, flags: Vec<String>) {
    let mut installed = false;
    if flags.contains(&"--installed".to_string()) || flags.contains(&"-i".to_string()) {
        installed = true;
    }
    println!(
        "novus {} \n\nPackages:\n",
        __VERSION__.bright_green().bold()
    );
    if installed {
        let temp = std::env::var("TEMP").unwrap();
        let loc = format!(r"{}\novus\config\installed.json", temp);
        let path = std::path::Path::new(loc.as_str());
        let contents = std::fs::read_to_string(path).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let json: Packages = serde_json::from_str::<Packages>(contents.as_str()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let installed_packages = json.clone().packages;
        for package in installed_packages {
            let package_split: Vec<&str> = package.split("@").collect();
            let pkg: &str = package_split[0];
            println!("  {}{}", " - ".bright_blue(), pkg.bright_cyan())
        }
    }
    else {
        for package in packages {
            println!("  {}{}", " - ".bright_blue(), package.bright_cyan())
        }
    }
    println!(
        "\nRun {} for more info about each command.",
        "novus [command] --help".bright_green()
    );
    std::process::exit(0);
}