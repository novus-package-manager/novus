use colored::Colorize;

pub fn handle_error_and_exit(e: String) -> ! {
    println!("{}{:?}", "error ".bright_red(), e);
    std::process::exit(0);
}
