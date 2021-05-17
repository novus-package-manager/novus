#[allow(unused)]
pub const COMMANDS: [&str; 11] = [
  "install",
  "uninstall",
  "update",
  "bundle",
  "search",
  "new",
  "config",
  "sign",
  "show",
  "find",
  "list",
];

#[allow(unused)]
pub const INSTALL_FLAGS: [&str; 17] = [
  "--verbose",
  "--debug",
  "--no-progress",
  "--no-color",
  "--log-output",
  "--install-dir",
  "--virus-check",
  "--yes",
  "--silent",
  "--vscode",
  "--python",
  "--node",
  "--sync",
  "--reduce",
  "--rate-limit",
  "--portable",
  "--manifest"
];

#[allow(unused)]
pub const UNINSTALL_FLAGS: [&str; 12] = [
  "--verbose",
  "--debug",
  "--no-color",
  "--log-output",
  "--yes",
  "--silent",
  "--vscode",
  "--python",
  "--node",
  "--no-cache",
  "--portable",
  "--manifest"
];

#[allow(unused)]
pub const LIST_FLAGS: [&str; 2] = ["--installed", "--versions"];

