use std::process::Command;

use crate::handle_error::handle_error_and_exit;

pub fn autoelevateinstall(package_name: String, mut args: Vec<String>) -> i32 {
    let batch_file = std::env::temp_dir().join(r"novus\scripts\auto_elevate_install.bat");
    args.insert(0, package_name);
    // println!("args: {:?}", args);
    let output = Command::new(batch_file)
        .args(args)
        .output()
        .unwrap_or_else(|_| handle_error_and_exit("Failed to retrieve output".to_string()));
    output.status.code().unwrap()
}
