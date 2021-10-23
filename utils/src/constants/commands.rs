#[allow(unused)]
pub const COMMANDS: [[&str; 2]; 13] = [
  ["install", "i"],
  ["uninstall", "u"],
  ["update", "upgrade"],
  ["clean", "clear"],
  ["search", "find"],
  ["config", "config"],
  ["list", "show"],
  ["quit", "exit"],
  ["forcequit", "forcequit"],
  ["info", "details"],
  ["startup", "startup"],
  ["status", "check"],
  ["alias", "alias"],
];

#[allow(unused)]
pub const ALL_COMMANDS: [&str; 22] = [
  "install",
  "i",
  "uninstall",
  "u",
  "update",
  "upgrade",
  "clean",
  "clear",
  "search",
  "find",
  "config",
  "list",
  "show",
  "quit",
  "exit",
  "forcequit",
  "info",
  "details",
  "startup",
  "check",
  "status",
  "alias",
];

pub const CONFIG_FLAGS: [&str; 9] = [
  "multithreaded",
  "no-color",
  "no-progress",
  "portable",
  "confirm",
  "reset",
  "installpath",
  "default",
  "list",
];

#[allow(unused)]
pub const INSTALL_FLAGS: [[&str; 2]; 5] = [
  ["--no-progress", "-np"],
  ["--no-color", "-nc"],
  ["--portable", "-p"],
  ["--multithreaded", "-m"],
  ["--installpath", "-path"],
];

#[allow(unused)]
pub const UNINSTALL_FLAGS: [[&str; 2]; 2] = [["--no-color", "-nc"], ["--portable", "-p"]];

#[allow(unused)]
pub const LIST_FLAGS: [[&str; 2]; 4] = [
  ["--installed", "-i"],
  ["--version", "-v"],
  ["--all", "-a"],
  ["--local", "-l"],
];

#[allow(unused)]
pub const INFO_FLAGS: [[&str; 2]; 1] = [["--local", "-l"]];

#[allow(unused)]
pub const SEARCH_FLAGS: [[&str; 2]; 5] = [
  ["--starts-with", "-sw"],
  ["--exact", "-e"],
  ["--installed", "-i"],
  ["--version", "-v"],
  ["--all", "-a"],
];

#[allow(unused)]
pub const CLEAN_FLAGS: [[&str; 2]; 2] = [["--yes", "-y"], ["--all", "-a"]];

#[allow(unused)]
pub const QUIT_FLAGS: [[&str; 2]; 2] = [["--yes", "-y"], ["--force", "-f"]];

#[allow(unused)]
pub const FORCEQUIT_FLAGS: [[&str; 2]; 1] = [["--yes", "-y"]];
