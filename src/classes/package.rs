use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Package {
  pub package_name: String,
  pub display_name: String,
  pub latest_version: String,
  pub home_page: String,
  pub threads: u64,
  pub iswitches: Vec<String>,
  pub uswitches: Vec<String>,
  pub autoupdate: AutoUpdateData,
  #[serde(flatten)]
  pub versions: HashMap<String, VersionData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AutoUpdateData {
  pub download_page: String,
  pub download_url: String,
  pub regex: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionData {
  pub url: String,
  pub size: u64,
  pub checksum: String,
  pub file_type: String,
}
