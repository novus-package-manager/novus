use super::handle_error::handle_error_and_exit;
use crate::classes::local_package_info::LocalPackageInfo;

use colored::Colorize;

pub fn check_installed(display_name: String) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    let mut regkey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut exists: bool = false;
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
                exists = true;
            }
        }
    }
    regkey = RegKey::predef(HKEY_LOCAL_MACHINE);
    if exists == false {
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
                exists = true;
            }
        }
    }

    if exists == false {
        let path: RegKey = regkey
            .open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        for name in path
            .enum_keys()
            .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
        {
            let unins_path: RegKey = regkey
                .open_subkey(format!(
                    "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}",
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
            // println!("app name 3: {}", app_name);
            if app_name
                .to_lowercase()
                .starts_with(display_name.to_lowercase().as_str())
            {
                exists = true;
            }
        }
    }

    exists
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
        let path: RegKey = regkey
            .open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        for name in path
            .enum_keys()
            .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
        {
            let unins_path: RegKey = regkey
                .open_subkey(format!(
                    "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}",
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


pub fn get_startup_apps() -> Vec<String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let regkey = RegKey::predef(HKEY_LOCAL_MACHINE);

    let path: RegKey = regkey
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let mut apps: Vec<String> = vec![];

    for (name, _val) in path
        .enum_values()
        .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
    {
        // println!("name: {}", name);
        apps.push(name);
    }

    apps
}

pub fn get_local_info(package_name: String) -> LocalPackageInfo {
    use winreg::enums::*;
    use winreg::RegKey;

    let mut regkey = RegKey::predef(HKEY_LOCAL_MACHINE);

    let mut local_package_info_global: LocalPackageInfo = LocalPackageInfo {
        comments: String::new(),
        contact: String::new(),
        display_name: String::new(),
        display_version: String::new(),
        help_link: String::new(),
        help_telephone: String::new(),
        install_date: String::new(),
        install_location: String::new(),
        install_source: String::new(),
        modify_path: String::new(),
        publisher: String::new(),
        readme: String::new(),
        size: String::new(),
        uninstall_string: String::new(),
        url_info_about: String::new(),
        url_update_info: String::new(),
    };
    let mut complete: bool = false;

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
                .unwrap_or("Unknown".to_string());
            if app_name
                .to_lowercase()
                .starts_with(package_name.to_lowercase().as_str())
            {
                let comments: String = unins_path
                    .get_value("Comments")
                    .unwrap_or("Unknown".to_string());
                let contact: String = unins_path
                    .get_value("Contact")
                    .unwrap_or("Unknown".to_string());
                let display_name: String = unins_path
                    .get_value("DisplayName")
                    .unwrap_or("Unknown".to_string());
                let display_version: String = unins_path
                    .get_value("DisplayVersion")
                    .unwrap_or("Unknown".to_string());
                let help_link: String = unins_path
                    .get_value("HelpLink")
                    .unwrap_or("Unknown".to_string());
                let help_telephone: String = unins_path
                    .get_value("HelpTelephone")
                    .unwrap_or("Unknown".to_string());
                let install_date: String = unins_path
                    .get_value("InstallDate")
                    .unwrap_or("Unknown".to_string());
                let install_location: String = unins_path
                    .get_value("InstallLocation")
                    .unwrap_or("Unknown".to_string());
                let install_source: String = unins_path
                    .get_value("InstallSource")
                    .unwrap_or("Unknown".to_string());
                let modify_path: String = unins_path
                    .get_value("ModifyPath")
                    .unwrap_or("Unknown".to_string());
                let publisher: String = unins_path
                    .get_value("Publisher")
                    .unwrap_or("Unknown".to_string());
                let readme: String = unins_path
                    .get_value("Readme")
                    .unwrap_or("Unknown".to_string());
                let size: String = unins_path
                    .get_value("Size")
                    .unwrap_or("Unknown".to_string());
                let uninstall_string: String = unins_path
                    .get_value("UninstallString")
                    .unwrap_or("Unknown".to_string());
                let url_info_about: String = unins_path
                    .get_value("URLInfoAbout")
                    .unwrap_or("Unknown".to_string());
                let url_update_info: String = unins_path
                    .get_value("URLUpdateInfo")
                    .unwrap_or("Unknown".to_string());
                let local_package_info = LocalPackageInfo {
                    comments: comments,
                    contact: contact,
                    display_name: display_name,
                    display_version: display_version,
                    help_link: help_link,
                    help_telephone: help_telephone,
                    install_date: install_date,
                    install_location: install_location,
                    install_source: install_source,
                    modify_path: modify_path,
                    publisher: publisher,
                    readme: readme,
                    size: size,
                    uninstall_string: uninstall_string,
                    url_info_about: url_info_about,
                    url_update_info: url_update_info,
                };
                local_package_info_global = local_package_info;
                complete = true
            }
        }
    }
    regkey = RegKey::predef(HKEY_LOCAL_MACHINE);
    if !complete {
        let path: RegKey = regkey
            .open_subkey(
                "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Installer\\UserData\\S-1-5-18\\Products",
            )
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        for name in path
            .enum_keys()
            .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
        {
            let unins_path: RegKey = regkey.open_subkey(format!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Installer\\UserData\\S-1-5-18\\Products\\{}\\InstallProperties", name)).unwrap_or(regkey.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall").unwrap_or_else(|e| handle_error_and_exit(e.to_string())));
            let app_name: String = unins_path
                .get_value("DisplayName")
                .unwrap_or("Unknown".to_string());
            if app_name
                .to_lowercase()
                .starts_with(package_name.to_lowercase().as_str())
            {
                let comments: String = unins_path
                    .get_value("Comments")
                    .unwrap_or("Unknown".to_string());
                let contact: String = unins_path
                    .get_value("Contact")
                    .unwrap_or("Unknown".to_string());
                let display_name: String = unins_path
                    .get_value("DisplayName")
                    .unwrap_or("Unknown".to_string());
                let display_version: String = unins_path
                    .get_value("DisplayVersion")
                    .unwrap_or("Unknown".to_string());
                let help_link: String = unins_path
                    .get_value("HelpLink")
                    .unwrap_or("Unknown".to_string());
                let help_telephone: String = unins_path
                    .get_value("HelpTelephone")
                    .unwrap_or("Unknown".to_string());
                let install_date: String = unins_path
                    .get_value("InstallDate")
                    .unwrap_or("Unknown".to_string());
                let install_location: String = unins_path
                    .get_value("InstallLocation")
                    .unwrap_or("Unknown".to_string());
                let install_source: String = unins_path
                    .get_value("InstallSource")
                    .unwrap_or("Unknown".to_string());
                let modify_path: String = unins_path
                    .get_value("ModifyPath")
                    .unwrap_or("Unknown".to_string());
                let publisher: String = unins_path
                    .get_value("Publisher")
                    .unwrap_or("Unknown".to_string());
                let readme: String = unins_path
                    .get_value("Readme")
                    .unwrap_or("Unknown".to_string());
                let size: String = unins_path
                    .get_value("Size")
                    .unwrap_or("Unknown".to_string());
                let uninstall_string: String = unins_path
                    .get_value("UninstallString")
                    .unwrap_or("Unknown".to_string());
                let url_info_about: String = unins_path
                    .get_value("URLInfoAbout")
                    .unwrap_or("Unknown".to_string());
                let url_update_info: String = unins_path
                    .get_value("URLUpdateInfo")
                    .unwrap_or("Unknown".to_string());
                let local_package_info = LocalPackageInfo {
                    comments: comments,
                    contact: contact,
                    display_name: display_name,
                    display_version: display_version,
                    help_link: help_link,
                    help_telephone: help_telephone,
                    install_date: install_date,
                    install_location: install_location,
                    install_source: install_source,
                    modify_path: modify_path,
                    publisher: publisher,
                    readme: readme,
                    size: size,
                    uninstall_string: uninstall_string,
                    url_info_about: url_info_about,
                    url_update_info: url_update_info,
                };
                local_package_info_global = local_package_info;
                complete = true
            }
        }
    }

    if !complete {
        let path: RegKey = regkey
            .open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        for name in path
            .enum_keys()
            .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
        {
            let unins_path: RegKey = regkey
                .open_subkey(format!(
                    "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}",
                    name
                ))
                .unwrap_or(
                    regkey
                        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
                        .unwrap_or_else(|e| handle_error_and_exit(e.to_string())),
                );
            let app_name: String = unins_path
                .get_value("DisplayName")
                .unwrap_or("Unknown".to_string());
            if app_name
                .to_lowercase()
                .starts_with(package_name.to_lowercase().as_str())
            {
                let comments: String = unins_path
                    .get_value("Comments")
                    .unwrap_or("Unknown".to_string());
                let contact: String = unins_path
                    .get_value("Contact")
                    .unwrap_or("Unknown".to_string());
                let display_name: String = unins_path
                    .get_value("DisplayName")
                    .unwrap_or("Unknown".to_string());
                let display_version: String = unins_path
                    .get_value("DisplayVersion")
                    .unwrap_or("Unknown".to_string());
                let help_link: String = unins_path
                    .get_value("HelpLink")
                    .unwrap_or("Unknown".to_string());
                let help_telephone: String = unins_path
                    .get_value("HelpTelephone")
                    .unwrap_or("Unknown".to_string());
                let install_date: String = unins_path
                    .get_value("InstallDate")
                    .unwrap_or("Unknown".to_string());
                let install_location: String = unins_path
                    .get_value("InstallLocation")
                    .unwrap_or("Unknown".to_string());
                let install_source: String = unins_path
                    .get_value("InstallSource")
                    .unwrap_or("Unknown".to_string());
                let modify_path: String = unins_path
                    .get_value("ModifyPath")
                    .unwrap_or("Unknown".to_string());
                let publisher: String = unins_path
                    .get_value("Publisher")
                    .unwrap_or("Unknown".to_string());
                let readme: String = unins_path
                    .get_value("Readme")
                    .unwrap_or("Unknown".to_string());
                let size: String = unins_path
                    .get_value("Size")
                    .unwrap_or("Unknown".to_string());
                let uninstall_string: String = unins_path
                    .get_value("UninstallString")
                    .unwrap_or("Unknown".to_string());
                let url_info_about: String = unins_path
                    .get_value("URLInfoAbout")
                    .unwrap_or("Unknown".to_string());
                let url_update_info: String = unins_path
                    .get_value("URLUpdateInfo")
                    .unwrap_or("Unknown".to_string());
                let local_package_info = LocalPackageInfo {
                    comments: comments,
                    contact: contact,
                    display_name: display_name,
                    display_version: display_version,
                    help_link: help_link,
                    help_telephone: help_telephone,
                    install_date: install_date,
                    install_location: install_location,
                    install_source: install_source,
                    modify_path: modify_path,
                    publisher: publisher,
                    readme: readme,
                    size: size,
                    uninstall_string: uninstall_string,
                    url_info_about: url_info_about,
                    url_update_info: url_update_info,
                };
                local_package_info_global = local_package_info;
                complete = true
            }
        }
    }

    if !complete {
        println!("{}\n", "Failed to locate package info".bright_cyan());
        std::process::exit(0);
    }

    local_package_info_global
}

pub fn get_local_packages() -> Vec<(String, String)> {
    use winreg::enums::*;
    use winreg::RegKey;

    let mut regkey = RegKey::predef(HKEY_LOCAL_MACHINE);

    let mut name_versions: Vec<(String, String)> = vec![];

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
            let display_name: String = unins_path
                .get_value("DisplayName")
                .unwrap_or("Unknown".to_string());
            let display_version: String = unins_path
                .get_value("DisplayVersion")
                .unwrap_or("Unknown".to_string());
            let name_version = (display_name, display_version);
            name_versions.push(name_version);
        }
    }
    regkey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path: RegKey = regkey
        .open_subkey(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Installer\\UserData\\S-1-5-18\\Products",
        )
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    for name in path
        .enum_keys()
        .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
    {
        let unins_path: RegKey = regkey.open_subkey(format!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Installer\\UserData\\S-1-5-18\\Products\\{}\\InstallProperties", name)).unwrap_or(regkey.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall").unwrap_or_else(|e| handle_error_and_exit(e.to_string())));
        let display_name: String = unins_path
            .get_value("DisplayName")
            .unwrap_or("Unknown".to_string());
        let display_version: String = unins_path
            .get_value("DisplayVersion")
            .unwrap_or("Unknown".to_string());
        let name_version = (display_name, display_version);
        name_versions.push(name_version);
    }
    let path: RegKey = regkey
        .open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    for name in path
        .enum_keys()
        .map(|x| x.unwrap_or_else(|e| handle_error_and_exit(e.to_string())))
    {
        let unins_path: RegKey = regkey
            .open_subkey(format!(
                "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}",
                name
            ))
            .unwrap_or(
                regkey
                    .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
                    .unwrap_or_else(|e| handle_error_and_exit(e.to_string())),
            );
        let display_name: String = unins_path
            .get_value("DisplayName")
            .unwrap_or("Unknown".to_string());
        let display_version: String = unins_path
            .get_value("DisplayVersion")
            .unwrap_or("Unknown".to_string());
        let name_version = (display_name, display_version);
        name_versions.push(name_version);
    }

    name_versions
}
