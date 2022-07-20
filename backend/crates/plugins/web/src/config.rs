use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ServerConfig {
    pub db_url: String,
    pub templates: String, // todo include
    pub statics_path: String,
}
