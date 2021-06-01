use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PackageInfo {
    pub package_name: String,
    pub display_name: String,
    pub exec_name: String,
    pub latest_version: String,
    pub threads: u64,
    pub iswitches: Vec<String>,
    pub uswitches: Vec<String>,
    pub url: String,
    pub size: u64,
    pub checksum: String,
    pub file_type: String,
}
