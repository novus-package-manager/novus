use colored::Colorize;
use serde_json::to_writer_pretty;
use std::collections::HashMap;
use std::fs::read_to_string;
use utils::classes::aliases::Aliases;
use utils::constants::help_menu::alias_package_error;
use utils::constants::version::__VERSION__;
use utils::handle_error::handle_error_and_exit;

pub async fn alias(args: Vec<String>, package_list: Vec<&str>) {
    let (package, value) = parse_args(args, package_list);
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate appdata directory".to_string())
    });
    let alias_file_loc = format!(r"{}\novus\aliases.json", appdata);
    let alias_file_path = std::path::Path::new(&alias_file_loc);

    let contents = read_to_string(&alias_file_path)
        .unwrap_or_else(|_| handle_error_and_exit("Failed to read alias file".to_string()));

    let mut json_content = serde_json::from_str::<Aliases>(&contents)
        .unwrap_or_else(|_| handle_error_and_exit("Failed to parse alias file".to_string()));

    let mut aliases: HashMap<String, Vec<String>> = json_content.aliases;

    let mut pkg_aliases: Vec<String> = vec![];

    for (key, val) in aliases.iter() {
        if key == &package {
            pkg_aliases = val.to_owned();
            println!("aliases for {}: {:?}", package, val);
        }
    }

    pkg_aliases.push(value.clone());

    if let Some(e) = aliases.get_mut(&package) {
        *e = pkg_aliases;
    } else {
        aliases.insert(package.clone(), vec![value]);
    }

    println!("{} aliases: {:?}", package, aliases);

    json_content.aliases = aliases;

    let file = std::fs::File::create(alias_file_loc)
        .unwrap_or_else(|_| handle_error_and_exit("Failed to create alias file".to_string()));
    to_writer_pretty(file, &json_content)
        .unwrap_or_else(|_| handle_error_and_exit("Failed to write alias file".to_string()));
}

fn parse_args(args: Vec<String>, package_list: Vec<&str>) -> (String, String) {
    let package: &String;
    let value: &String;
    if args.len() < 4 {
        alias_package_error();
        std::process::exit(1);
    } else {
        package = &args[2];
        value = &args[3];
    }

    if !package_list.contains(&package.as_str()) {
        let package_error = format!(
            r#"
Novus Package Manager {}

{} Couldn't find package {}
        
{} Use {} to view the list of packages."#,
            __VERSION__.bright_green().bold(),
            "error".bright_red(),
            package.bright_cyan(),
            "info".bright_blue(),
            "novus list".bright_green()
        );
        println!("{}", package_error);
        std::process::exit(1);
    }

    (package.to_owned(), value.to_owned())
}
