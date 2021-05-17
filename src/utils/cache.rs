#[allow(unused)]
pub fn check_cache(package_name: String, version: String) -> bool {
  let temp = std::env::var("TEMP").unwrap();
  let loc = format!(r"{}\novus\{}@{}.exe", temp, package_name, version);
  let path = std::path::Path::new(loc.as_str());
  path.exists()
}