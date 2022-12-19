use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Config {
    pub paths_to_check: Vec<String>,
}
