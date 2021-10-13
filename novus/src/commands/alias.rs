use std::collections::HashMap;
use utils::handle_error::handle_error_and_exit;
use colored::Colorize;
use utils::classes::aliases::Aliases;
use std::fs::read_to_string;

pub async fn alias(args: Vec<String>) {
    // println!("args: {:?}", args);
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate appdata directory".to_string())
    });
    let alias_file_loc = format!(r"{}\novus\aliases.json", appdata);
    let alias_file_path = std::path::Path::new(&alias_file_loc);

    let contents = read_to_string(&alias_file_path).unwrap_or_else(|_| {
        handle_error_and_exit("Failed to read alias file".to_string())
    });

    let aliases = serde_json::from_str::<Aliases>(&contents).unwrap_or_else(|_| {
        handle_error_and_exit("Failed to parse alias file".to_string())
    });

    let aliases: Vec<HashMap<String, Vec<String>>> = aliases.aliases;

    for hashmap in aliases.iter() {
        for (key, val) in hashmap.iter() {
            if key == "brave" {
                println!("val for brave: {:?}", val);
            }
        }
    }

}

// fn parse_args(args: Vec<String>) -> (String, String) {
    
// }