use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub multithreaded: bool,
    pub no_color: bool,
    pub no_progress: bool,
    pub portable: bool,
    pub confirm: bool,
    pub installpath: String,
}
