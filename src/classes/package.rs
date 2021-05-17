use serde::Deserialize;
use std::collections::HashMap;
#[derive(Deserialize, Debug)]
pub struct Package {
  pub package_name: String,
  pub display_name: String,
  pub latest_version: String,
  pub creator: String,
  pub home_page: String,
  #[serde(flatten)]
  pub versions: HashMap<String, VersionData>,
}

#[derive(Deserialize, Debug)]
pub struct VersionData  {
  pub url: String,
  pub threads: u64,
  pub size: u64,
  pub file_type: String,
  pub iswitches: Vec<String>,
  pub uswitches: Vec<String>,
  pub checksum: String,
}