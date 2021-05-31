use crate::utils::handle_error::handle_error_and_exit;

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
