use colored::Colorize;
use utils::classes::local_package_info::LocalPackageInfo;
use utils::classes::package::Package;
use utils::classes::package_info::PackageInfo;
use utils::constants::help_menu::info_wrong_package_error;
use utils::constants::version::__VERSION__;
use utils::get_package::get_package;
use utils::registry::get_local_info;

pub async fn info(args: Vec<String>, packages: Vec<&str>) {
    println!(
        "\nNovus Package Manager {}\n",
        __VERSION__.bright_green().bold()
    );

    let (package, flags) = verify_args(args, packages);

    let mut local = false;
    if flags.contains(&"--local".to_string()) || flags.contains(&"-l".to_string()) {
        local = true;
    }
    if local {
        let package_info: LocalPackageInfo = get_local_info(package);
        println!("");
        if package_info.comments != "Unknown" && package_info.comments != "" {
            print!("{}{:<40}", " - ".bright_blue(), "Comments".bright_cyan());
            println!("{}", package_info.comments);
        }
        if package_info.contact != "Unknown" && package_info.contact != "" {
            print!("{}{:<40}", " - ".bright_blue(), "Contact".bright_cyan());
            println!("{}", package_info.contact);
        }
        if package_info.display_name != "Unknown" && package_info.display_name != "" {
            print!("{}{:<40}", " - ".bright_blue(), "DisplayName".bright_cyan());
            println!("{}", package_info.display_name);
        }
        if package_info.display_version != "Unknown" && package_info.display_version != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "DisplayVersion".bright_cyan()
            );
            println!("{}", package_info.display_version);
        }
        if package_info.help_link != "Unknown" && package_info.help_link != "" {
            print!("{}{:<40}", " - ".bright_blue(), "HelpLink".bright_cyan());
            println!("{}", package_info.help_link);
        }
        if package_info.help_telephone != "Unknown" && package_info.help_telephone != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "HelpTelephone".bright_cyan()
            );
            println!("{}", package_info.help_telephone);
        }
        if package_info.install_date != "Unknown" && package_info.install_date != "" {
            print!("{}{:<40}", " - ".bright_blue(), "InstallDate".bright_cyan());
            println!("{}", package_info.install_date);
        }
        if package_info.install_location != "Unknown" && package_info.install_location != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "InstallLocation".bright_cyan()
            );
            println!("{}", package_info.install_location);
        }
        if package_info.install_source != "Unknown" && package_info.install_source != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "InstallSource".bright_cyan()
            );
            println!("{}", package_info.install_source);
        }
        if package_info.modify_path != "Unknown" && package_info.modify_path != "" {
            print!("{}{:<40}", " - ".bright_blue(), "ModifyPath".bright_cyan());
            println!("{}", package_info.modify_path);
        }
        if package_info.publisher != "Unknown" && package_info.publisher != "" {
            print!("{}{:<40}", " - ".bright_blue(), "Publisher".bright_cyan());
            println!("{}", package_info.publisher);
        }
        if package_info.readme != "Unknown" && package_info.readme != "" {
            print!("{}{:<40}", " - ".bright_blue(), "Readme".bright_cyan());
            println!("{}", package_info.readme);
        }
        if package_info.size != "Unknown" && package_info.size != "" {
            print!("{}{:<40}", " - ".bright_blue(), "Size".bright_cyan());
            println!("{}", package_info.size);
        }
        if package_info.uninstall_string != "Unknown" && package_info.uninstall_string != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "UninstallString".bright_cyan()
            );
            println!("{}", package_info.uninstall_string);
        }
        if package_info.url_info_about != "Unknown" && package_info.url_info_about != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "URLInfoAbout".bright_cyan()
            );
            println!("{}", package_info.url_info_about);
        }
        if package_info.url_update_info != "Unknown" && package_info.url_update_info != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "URLUpdateInfo".bright_cyan()
            );
            println!("{}", package_info.url_update_info);
        }
        println!("");
    } else {
        let package_info = get_package_info(package).await;
        if package_info.package_name != "Unknown" && package_info.package_name != "" {
            print!("{}{:<40}", " - ".bright_blue(), "PackageName".bright_cyan());
            println!("{}", package_info.package_name);
        }
        if package_info.display_name != "Unknown" && package_info.display_name != "" {
            print!("{}{:<40}", " - ".bright_blue(), "DisplayName".bright_cyan());
            println!("{}", package_info.display_name);
        }
        if package_info.exec_name != "Unknown" && package_info.exec_name != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "ExecutableName".bright_cyan()
            );
            println!("{}", package_info.exec_name);
        }
        if package_info.latest_version != "Unknown" && package_info.latest_version != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "LatestVersion".bright_cyan()
            );
            println!("{}", package_info.latest_version);
        }
        if package_info.threads != 0 {
            print!("{}{:<40}", " - ".bright_blue(), "Threads".bright_cyan());
            println!("{}", package_info.threads);
        }
        if package_info.iswitches.len() != 0 {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "InstallSwitches".bright_cyan()
            );
            println!("{:?}", package_info.iswitches);
        }
        if package_info.uswitches.len() != 0 {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "UninstallSwitches".bright_cyan()
            );
            println!("{:?}", package_info.uswitches);
        }
        if package_info.url != "Unknown" && package_info.url != "" {
            print!("{}{:<40}", " - ".bright_blue(), "Url".bright_cyan());
            println!("{}", package_info.url);
        }
        if package_info.size != 0 {
            print!("{}{:<40}", " - ".bright_blue(), "Size".bright_cyan());
            println!("{}", package_info.size);
        }
        if package_info.checksum != "Unknown" && package_info.checksum != "" {
            print!(
                "{}{:<40}",
                " - ".bright_blue(),
                "Checksum (SHA256)".bright_cyan()
            );
            println!("{}", package_info.checksum);
        }
        if package_info.file_type != "Unknown" && package_info.file_type != "" {
            print!("{}{:<40}", " - ".bright_blue(), "FileType".bright_cyan());
            println!("{}", package_info.file_type);
        }
        println!("");
    }
}

async fn get_package_info(package_name: String) -> PackageInfo {
    let package: Package = get_package(&package_name).await;
    PackageInfo {
        package_name: package.package_name,
        display_name: package.display_name,
        exec_name: package.exec_name,
        latest_version: package.latest_version.clone(),
        threads: package.threads,
        iswitches: package.iswitches,
        uswitches: package.uswitches,
        url: package.versions[&package.latest_version].url.clone(),
        size: package.versions[&package.latest_version].size.clone(),
        checksum: package.versions[&package.latest_version].checksum.clone(),
        file_type: package.versions[&package.latest_version].file_type.clone(),
    }
}

fn verify_args(args: Vec<String>, packages: Vec<&str>) -> (String, Vec<String>) {
    let mut flags = vec![];
    let mut package = String::new();
    for arg in args {
        if arg.starts_with("-") {
            flags.push(arg);
        } else {
            package = arg;
        }
    }

    if !flags.contains(&"-l".to_string()) && !flags.contains(&"--local".to_string()) {
        if !packages.contains(&package.as_str()) {
            info_wrong_package_error();
        }
    }

    (package, flags)
}
