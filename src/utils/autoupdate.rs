use crate::classes::package::{Package, VersionData};
use crate::utils::{get_package::get_package, handle_error::handle_error_and_exit, checksum::get_checksum};
use reqwest::blocking::get;
use crate::commands::install::threadeddownload;

pub fn get_latest_version(package_name: &str) {
    let package: Package = get_package(package_name.clone());
    let mut temp_package: Package = package.clone();
    let url = package.autoupdate.download_page;
    println!("url: {}", url);
    let mut file_contents = String::new();
    match get("https://www.7-zip.org/download.html") {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => {
                        file_contents = text;
                    }
                    Err(err) => eprintln!("Could Not Read Response JSON, {}", err),
                }
            } else {
                println!("Failed To Send Request");
                std::process::exit(1);
            }
        }
        Err(err) => eprintln!("Failed To Send Request: {}", err),
    }

    let regex = regex::Regex::new(package.autoupdate.regex.as_str()).unwrap();

    let years: Vec<&str> = regex.captures_iter(file_contents.as_str()).map(|c| c.get(1).unwrap().as_str()).collect();
    // println!("years: {:?}", years);
    let index = years.iter()
    .enumerate()
    .filter_map(|(i, s)| s.parse::<u64>().ok().map(|n| (i, n)))
    .max_by_key(|&(_, n)| n)
    .map(|(i, _)| i).unwrap_or_else(|| handle_error_and_exit("Failed to find match".to_string()));    
    let versions: Vec<&str> = regex.captures_iter(file_contents.as_str()).map(|c| c.get(0).unwrap().as_str()).collect();
    let mut versions: Vec<&str> = versions[index].split("(").collect();
    let mut version = versions[0];
    if versions[0].contains("alpha") {
        versions = versions[0].split("alpha").collect();
        version = versions[0];
    }
    let version = version.replace(" ", "");
    println!("version: {:?}", version);

    if package.latest_version != version {
        let url = "https://www.7-zip.org/a/7z".to_string() + version.replace(".", "").as_str() + "-x64.exe";
        let mut file_size: u64 = 0;
        match get(url.clone()) {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::OK {
                    file_size = response.content_length().unwrap();
                } else {
                    println!("Failed To Send Request");
                    std::process::exit(1);
                }
            }
            Err(err) => eprintln!("Failed To Send Request: {}", err),
        }

        let temp = std::env::var("TEMP").unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let loc = format!(r"{}\novus\{}_check.exe", temp, package_name);
        threadeddownload(url.clone(), loc.clone(), package.threads, package_name.to_string(), "".to_string(), false, false);
        let hash = get_checksum(loc.clone());

        let _ = std::fs::remove_file(loc);

        let version_data: VersionData = VersionData {
            url: url,
            size: file_size,
            checksum: hash,
        };

        println!("version_data: {:?}", version_data);

        // make changes to data
        temp_package.versions.insert(version.clone(), version_data);
        temp_package.latest_version = version;

        // Re-open file to replace the contents:
        let file = std::fs::File::create(format!("../novus-packages/packages/{}.json", package_name)).unwrap(); 
        serde_json::to_writer_pretty(file, &temp_package).unwrap();

        let dir = std::path::Path::new(r"D:\prana\Programming\My Projects\novus-package-manager\novus-packages");
        let _ = std::env::set_current_dir(dir);
        let mut commit = format!("autoupdater: Updated {}", package_name);
        commit = "\"".to_string() + commit.as_str() + "\"";
        std::process::Command::new("powershell").args(&["git", "add", "."]).output().expect("Failed to add");
        std::process::Command::new("powershell").args(&["git", "commit", "-m", commit.as_str()]).output().expect("Failed to commit");
        std::process::Command::new("powershell").args(&["git", "push"]).output().expect("Failed to push");
    }
}