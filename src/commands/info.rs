use crate::classes::local_package_info::LocalPackageInfo;
use crate::constants::version::__VERSION__;
use crate::utils::registry::get_local_info;
use colored::Colorize;

pub fn info(args: Vec<String>) {
    println!(
        "\nNovus Package Manager {}\n",
        __VERSION__.bright_green().bold()
    );

    let (package, flags) = verify_args(args);

    let mut local = false;
    if flags.contains(&"--local".to_string()) || flags.contains(&"-l".to_string()) {
        local = true;
    }
    if local {
        let package_info: LocalPackageInfo = get_local_info(package);
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
    }
}

fn verify_args(args: Vec<String>) -> (String, Vec<String>) {
    let mut flags = vec![];
    let mut package = String::new();
    for arg in args {
        if arg.starts_with("-") {
            flags.push(arg);
        } else {
            package = arg;
        }
    }

    (package, flags)
}
