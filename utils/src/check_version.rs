use crate::handle_error::handle_error_and_exit;
use crate::constants::version::VERSION_NUM;

pub async fn check_version() -> bool {
    let text: String =
        reqwest::get("https://github.com/novus-package-manager/novus/releases/latest")
            .await
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
            .text()
            .await
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    let regex = regex::Regex::new("Novus Alpha v(\\d+\\.\\d+\\.\\d+)")
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let matches: Vec<&str> = regex
        .captures_iter(text.as_str())
        .map(|c| c.get(1).unwrap().as_str())
        .collect();

    let mut versions: Vec<String> = vec![];
    for mut _match in matches.clone() {
        versions.push(_match.replace(".", ""));
    }

    let index = versions
        .iter()
        .enumerate()
        .filter_map(|(i, s)| s.parse::<u64>().ok().map(|n| (i, n)))
        .max_by_key(|&(_, n)| n)
        .map(|(i, _)| i)
        .unwrap_or_else(|| handle_error_and_exit("Failed to find match".to_string()));

    let version = matches[index];
    let version_number: u64 = version
        .replace(".", "")
        .parse::<u64>()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    if version_number > VERSION_NUM {
        return true;
    }

    false
}