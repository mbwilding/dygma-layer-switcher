use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppDetails {
    pub window: Option<String>,
    pub process: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub layer: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}
