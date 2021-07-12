use std::path::Path;
use std::process::Command;

use crate::handle_error::handle_error_and_exit;

pub fn autoelevateinstall(package_name: String) -> i32 {
    let batch_file_loc = format!(
        "{}\\{}",
        std::env::var("APPDATA")
            .unwrap_or_else(|_| handle_error_and_exit("Failed to locate AppData dir".to_string())),
        (r"novus\scripts\auto_elevate_install.bat")
    );

    let batch_file = Path::new(&batch_file_loc);
    // args.insert(0, package_name);
    // args.push("-y".to_string());
    let output = Command::new(batch_file)
        .arg(package_name)
        .output()
        .unwrap_or_else(|_| handle_error_and_exit("Failed to retrieve output".to_string()));
    output.status.code().unwrap()
}

pub fn autoelevateuninstall(package_name: String) -> i32 {
    let batch_file_loc = format!(
        "{}\\{}",
        std::env::var("APPDATA")
            .unwrap_or_else(|_| handle_error_and_exit("Failed to locate AppData dir".to_string())),
        (r"novus\scripts\auto_elevate_uninstall.bat")
    );

    let batch_file = Path::new(&batch_file_loc);
    let output = Command::new(batch_file)
        .arg(package_name)
        .output()
        .unwrap_or_else(|_| handle_error_and_exit("Failed to retrieve output".to_string()));
    output.status.code().unwrap()
}
