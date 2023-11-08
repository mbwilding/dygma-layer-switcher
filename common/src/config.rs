use crate::app::App;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use tracing::{error, info, warn};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub comm_port: String,
    pub base_layer: u8,
    pub mappings: Vec<Application>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    pub layer: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exe_name: Option<String>,
}

impl Config {
    pub fn get_config() -> Config {
        let config_path = "config.yml";
        let file = File::open(config_path);
        let config = match file {
            Ok(mut f) => {
                let mut contents = String::new();
                match f.read_to_string(&mut contents) {
                    Ok(_) => match serde_yaml::from_str(&contents) {
                        Ok(data) => {
                            info!("Config loaded");
                            data
                        }
                        Err(e) => {
                            error!("Error parsing config file: {:?}", e);
                            Config::default()
                        }
                    },
                    Err(e) => {
                        info!("Error reading config file: {:?}", e);
                        Config::default()
                    }
                }
            }
            Err(_) => {
                warn!("No config file found, creating with default config");
                let default_config = Config::default();
                match File::create(config_path) {
                    Ok(mut file) => {
                        let yaml = serde_yaml::to_string(&default_config)
                            .expect("Failed to serialize default config");
                        file.write_all(yaml.as_bytes())
                            .expect("Failed to write default config to file");
                    }
                    Err(e) => {
                        error!("Failed to create config file: {:?}", e);
                    }
                }

                default_config
            }
        };

        config
    }

    pub fn check_exe_name(&self, app: &App) -> Option<u8> {
        self.mappings.iter().find_map(|mapping| {
            match (&app.exe_name, &mapping.exe_name) {
                // If both executable names are present and they match, return the layer number.
                (Some(app_exe_name), Some(mapping_exe_name))
                    if app_exe_name == mapping_exe_name =>
                {
                    Some(mapping.layer)
                }
                // If there is no match, continue to the next mapping.
                _ => None,
            }
        })
    }

    pub fn check_window_title(&self, app: &App) -> Option<u8> {
        self.mappings.iter().find_map(|mapping| {
            match (&app.window_title, &mapping.window_title) {
                // If both window titles are present and they match, return the layer number.
                (Some(app_window_title), Some(mapping_window_title))
                    if app_window_title == mapping_window_title =>
                {
                    Some(mapping.layer)
                }
                // If there is no match, continue to the next mapping.
                _ => None,
            }
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            comm_port: String::from("COM5"),
            base_layer: 0,
            mappings: vec![],
        }
    }
}
