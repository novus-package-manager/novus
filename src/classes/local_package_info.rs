use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LocalPackageInfo {
    pub comments: String,
    pub contact: String,
    pub display_name: String,
    pub display_version: String,
    pub help_link: String,
    pub help_telephone: String,
    pub install_date: String,
    pub install_location: String,
    pub install_source: String,
    pub modify_path: String,
    pub publisher: String,
    pub readme: String,
    pub size: String,
    pub uninstall_string: String,
    pub url_info_about: String,
    pub url_update_info: String,
}
