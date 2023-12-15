use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Layer {
    pub name: String,
    pub apps: Vec<App>,

    #[serde(skip)]
    pub is_editing: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub mode: Mode,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Mode {
    Window(Window),
    Process(Process),
    Parent(Parent),
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Window {
    pub name: String,
    pub is_editing: bool,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Process {
    pub name: String,
    pub is_editing: bool,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Parent {
    pub process: String,
    pub excludes: Vec<String>,

    #[serde(skip)]
    pub is_editing: bool,
}
