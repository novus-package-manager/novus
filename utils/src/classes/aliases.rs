use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct Aliases {
//     pub aliases: Vec<HashMap<String, Vec<String>>>
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Aliases {
//     pub aliases: Vec<Alias>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Alias {
//     pub brave: String,
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Aliases { 
    pub aliases: Vec<HashMap<String, Vec<String>>>,
}
