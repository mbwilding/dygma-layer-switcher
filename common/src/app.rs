use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AppDetails {
    pub window: Option<String>,
    pub process: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Layer {
    pub layer: u8,
    pub apps: Vec<AppConfig>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Parent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
}
