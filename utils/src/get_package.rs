use crate::classes::package::Package;
use crate::handle_error::handle_error_and_exit;
use reqwest::get;
use serde_json::{from_str, to_string_pretty, Value};

#[allow(unused)]
pub async fn get_package(package_name: &str) -> Package {
    let mut file_contents = String::new();
    let response = get(format!(
        "https://storage.googleapis.com/novus_bucket/{}.json?a={:?}",
        package_name,
        std::time::UNIX_EPOCH.elapsed().unwrap()
    ))
    .await
    .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    file_contents = response
        .text()
        .await
        .unwrap_or_else(|e| handle_error_and_exit(format!("{} get_package.rs:36", e.to_string())));
    from_str::<Package>(&file_contents).unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
}

#[allow(unused)]
pub async fn get_packages() -> String {
    let mut file_contents = String::new();
    let response = get(format!(
        "https://storage.googleapis.com/novus_bucket/package-list/package-list.json?a={:?}",
        std::time::UNIX_EPOCH.elapsed().unwrap()
    ))
    .await
    .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    file_contents = response
        .text()
        .await
        .unwrap_or_else(|e| handle_error_and_exit(format!("{} get_package.rs:36", e.to_string())));
    let content: Value = from_str(file_contents.as_str())
        .unwrap_or_else(|e| handle_error_and_exit(format!("{} get_package.rs:53", e.to_string())));
    to_string_pretty(&content)
        .unwrap_or_else(|e| handle_error_and_exit(format!("{} get_package.rs:54", e.to_string())))
}
