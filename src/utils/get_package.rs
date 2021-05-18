use std::process;
use crate::classes::package::Package;
use reqwest::blocking::get;
use serde_json::{from_str, to_string_pretty, Value};
use crate::utils::handle_error::handle_error_and_exit;

#[allow(unused)]
pub fn get_package(package_name: &str) -> Package {
//     let mut file_contents = String::new();
//     match get(format!("https://raw.githubusercontent.com/novus-package-manager/novus-packages/master/packages/{}.json", package_name)) {
//     Ok(response) => {
//         if response.status() == reqwest::StatusCode::OK {
//             match response.text() {
//                 Ok(text) => {
//                     file_contents = text;
//                 }
//                 Err(err) => eprintln!("Could Not Read Response JSON, {}", err),
//             }
//         } else {
//             println!("Response Was Not 200 OK");
//         }
//     }
//     Err(err) => eprintln!("Failed To Send Request: {}", err),
//   }
    // let file_contents = read_to_string(loc).unwrap();
    // from_str::<Package>(&file_contents).unwrap()
    let loc = format!(r"../novus-packages/packages/{}.json", package_name);
    let file_contents = std::fs::read_to_string(loc).unwrap_or_else(|e| handle_error_and_exit(format!("{} get_package.rs:28", e.to_string())));
    from_str::<Package>(&file_contents).unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
}

#[allow(unused)]
pub fn get_packages() -> String {
    let mut file_contents = String::new();
    match get("https://raw.githubusercontent.com/novus-package-manager/novus-packages/master/package-list.json") {
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
            process::exit(1);
        }
    }
    Err(err) => eprintln!("Failed To Send Request: {}", err),
  }
    let content: Value = from_str(file_contents.as_str()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    to_string_pretty(&content).unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
    // let loc = format!(r"../novus-packages/package-list.json");
    // std::fs::read_to_string(loc).unwrap()
    // from_str::<Package>(&file_contents).unwrap()
}
