use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Layer {
    pub layer: u8,
    pub apps: Vec<App>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct App {
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
