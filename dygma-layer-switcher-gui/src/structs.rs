use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Layer {
    pub name: String,
    pub apps: Vec<App>,

    #[serde(skip)]
    pub is_being_renamed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub mode: Mode,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Mode {
    Window(String),
    Process(String),
    Parent(Parent),
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Parent {
    pub process: String,
    pub excludes: Vec<String>,
}
