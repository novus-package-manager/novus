use colored::Colorize;
use sha2::{Digest, Sha256};

pub fn verify_checksum(output: String, checksum: String, no_color: bool) -> bool {
    let mut file = std::fs::File::open(output.clone()).unwrap();
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).unwrap();
    let hash = format!("{:x}", hasher.finalize());

    if hash.to_uppercase() == checksum.to_uppercase() {
        // Verified Checksum
        if no_color {
            println!("Successfully Verified Hash");
        } else {
            println!("{}", "Successfully Verified Hash".bright_green());
        }

        return true;
    } else {
        if no_color {
            println!("Failed To Verify Hash");
            println!("Clearing cache and retrying");
        }
        else {
            println!("{}", "Failed To Verify Hash".bright_red());
        }
        return false;
    }
}

#[allow(unused)]
pub fn get_checksum(output: String) -> String {
    let mut file = std::fs::File::open(output.clone()).unwrap();
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).unwrap();
    format!("{:x}", hasher.finalize()).to_uppercase()
}
