use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Aliases {
    pub aliases: HashMap<String, Vec<String>>,
}
