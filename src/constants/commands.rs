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
pub const INSTALL_FLAGS: [[&str; 2]; 8] = [
  ["--verbose", "-v"],
  ["--debug", "-d"],
  ["--no-progress", "-np"],
  ["--no-color", "-nc"],
  ["--log-output", "-lo"],  
  ["--virus-check", "-vc"],
  ["--yes", "-y"],
  ["--silent", "-s"]   
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
pub const LIST_FLAGS: [[&str; 2]; 2] = [["--installed", "-i"], ["--versions", "-v"]];

