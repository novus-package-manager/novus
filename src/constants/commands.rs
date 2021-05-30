#[allow(unused)]
pub const COMMANDS: [[&str; 2]; 9] = [
  ["install", "i"],
  ["uninstall", "u"],
  ["update", "updgrade"],
  ["clean", "clear"],
  ["search", "find"],
  ["config", "config"],
  ["list", "show"],
  ["quit", "exit"],
  ["forcequit", "forcequit"],
];

#[allow(unused)]
pub const ALL_COMMANDS: [&str; 16] = [
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
];

#[allow(unused)]
pub const INSTALL_FLAGS: [[&str; 2]; 8] = [
  ["--verbose", "-v"],
  ["--debug", "-d"],
  ["--no-progress", "-np"],
  ["--no-color", "-nc"],
  ["--log-output", "-lo"],
  ["--virus-check", "-vc"],
  ["--yes", "-y"],
  ["--silent", "-s"],
];

#[allow(unused)]
pub const UNINSTALL_FLAGS: [[&str; 2]; 7] = [
  ["--verbose", "-v"],
  ["--debug", "-d"],
  ["--no-color", "-nc"],
  ["--log-output", "-lg"],
  ["--yes", "-y"],
  ["--silent", "-s"],
  ["--no-cache", "--no-cache"],
];

#[allow(unused)]
pub const LIST_FLAGS: [[&str; 2]; 3] =
  [["--installed", "-i"], ["--version", "-v"], ["--all", "-a"]];

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
