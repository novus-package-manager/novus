use sha2::{Sha256, Digest};
use colored::Colorize;

pub fn verify_checksum(output: String, checksum: String) {
    let mut file = std::fs::File::open(output.clone()).unwrap();
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).unwrap();
    let hash = format!("{:x}", hasher.finalize());

    if hash.to_uppercase() == checksum.to_uppercase() {
        // Verified Checksum
        println!("{}", "Successfully Verified Hash".bright_green());
    } else {
        println!("{}", "Failed To Verify Hash".bright_red())
    }
}