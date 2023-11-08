use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub window_title: Option<String>,
    pub exe_name: Option<String>,
}
