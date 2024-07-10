use crate::verbiage;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Mode {
    Window(Window),
    Process(Process),
    Parent(Parent),
}

pub struct Configuration {
    pub port: String,
    pub base_layer: u8,
    pub mappings: BTreeMap<u8, Layer>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct AppDetails {
    pub window: String,
    pub process: String,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Window {
    pub name: String,

    #[serde(skip)]
    pub is_editing: bool,
}

impl Window {
    pub fn new() -> Self {
        Self {
            name: verbiage::EDIT_TEXT.to_string(),
            is_editing: false,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Process {
    pub name: String,

    #[serde(skip)]
    pub is_editing: bool,
}

impl Process {
    pub fn new() -> Self {
        Self {
            name: verbiage::EDIT_TEXT.to_string(),
            is_editing: false,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Parent {
    pub name: String,
    pub excludes: Vec<Exclude>,

    #[serde(skip)]
    pub is_editing: bool,
}

impl Parent {
    pub fn new() -> Self {
        Self {
            name: verbiage::EDIT_TEXT.to_string(),
            excludes: vec![],
            is_editing: false,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Exclude {
    pub name: String,
    pub is_enabled: bool,

    #[serde(skip)]
    pub is_editing: bool,
}

impl Exclude {
    pub fn new() -> Self {
        Self {
            name: verbiage::EDIT_TEXT.to_string(),
            is_enabled: true,
            is_editing: false,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            port: String::new(),
            base_layer: 1,
            mappings: BTreeMap::new(),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Layer {
    pub name: String,
    pub apps: Vec<App>,

    #[serde(skip)]
    pub is_editing: bool,
}

impl Layer {
    pub fn new(layer: u8) -> Self {
        Self {
            name: format!("Layer {}", layer + 1),
            apps: vec![],
            is_editing: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub mode: Mode,
    pub is_enabled: bool,
}

impl App {
    pub fn new_window() -> Self {
        Self {
            mode: Mode::Window(Window::new()),
            is_enabled: true,
        }
    }

    pub fn new_process() -> Self {
        Self {
            mode: Mode::Process(Process::new()),
            is_enabled: true,
        }
    }

    pub fn new_parent() -> Self {
        Self {
            mode: Mode::Parent(Parent::new()),
            is_enabled: true,
        }
    }
}
