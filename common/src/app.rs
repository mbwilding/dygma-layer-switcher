use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppDetails {
    pub window_title: Option<String>,
    pub exe_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub layer: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exe_name: Option<String>,
}
