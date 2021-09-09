use colored::Colorize;
use utils::classes::package::Package;
use utils::get_package::get_package;
use utils::registry::check_installed;

pub async fn status(_flags: Vec<String>, package_name: &str) {
    let package: Package = get_package(package_name).await;
    let display_name = package.display_name;

    let installed: bool = check_installed(display_name.clone());

    if installed {
        println!(
            "{}{}",
            display_name.bright_green(),
            " is installed on your system".bright_green()
        );
    } else {
        println!(
            "{}{}",
            display_name.bright_red(),
            " is not installed on your system".bright_red()
        );
    }

    std::process::exit(0);
}
