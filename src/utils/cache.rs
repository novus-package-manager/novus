#[allow(unused)]
pub fn check_cache(package_name: String, version: String, file_type: String) -> bool {
  let temp = std::env::var("TEMP").unwrap();
  let loc = format!(r"{}\novus\{}@{}{}", temp, package_name, version, file_type);
  let path = std::path::Path::new(loc.as_str());
  path.exists()
}

#[allow(unused)]
pub fn delete_temp_cache(package_name: String, threads: u64) {
  let temp = std::env::var("TEMP").unwrap();
  for index in 0..threads {
    let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index + 1);
    let _ = std::fs::remove_file(loc);
  }
}
