use std::fs;
use std::path::Path;

use crate::handle_error::handle_error_and_exit;

#[allow(unused)]
pub fn check_cache(package_name: String, version: String, file_type: String) -> bool {
    let appdata = std::env::var("APPDATA").unwrap();
    let loc = format!(r"{}\novus\{}@{}{}", appdata, package_name, version, file_type);
    let path = Path::new(loc.as_str());
    path.exists()
}

pub fn clear_cache() {
    let appdata = std::env::var("APPDATA")    
        .unwrap_or_else(|_| handle_error_and_exit("Failed to locate Appdata directory".to_string()));
        println!("appdata: {:?}", appdata);
    let loc = format!("{}/novus", appdata);
    let novus_dir = Path::new(&loc);
    if novus_dir.exists() {
        fs::remove_dir_all(novus_dir).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    }
}

pub fn clear_cache_for_package(package: &str) {
    let appdata = std::env::var("APPDATA")
        .unwrap_or_else(|_| handle_error_and_exit("Failed to locate Appdata directory".to_string()));
    let loc = format!("{}/novus", appdata);
    let novus_dir = Path::new(&loc);
    for file in fs::read_dir(novus_dir).unwrap() {
        let path = file.unwrap().path().display().to_string();
        let path_split: Vec<&str> = path.split("novus\\").collect();
        let file_name = path_split[1];
        if file_name.starts_with(&package.clone()) {
            let path = novus_dir.join(file_name.clone());
            if path.exists() {
                fs::remove_file(path).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
            }
        }
    }
}

#[allow(unused)]
pub fn delete_temp_cache(package_name: String, threads: u64) {
    let appdata = std::env::var("APPDATA").unwrap();
    for index in 0..threads {
        let loc = format!(r"{}\novus\setup_{}{}.tmp", appdata, package_name, index + 1);
        let _ = fs::remove_file(loc);
    }
}
