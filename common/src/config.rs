use crate::app::{AppConfig, AppDetails};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use tracing::{debug, error, warn};

const CONFIG_PATH: &str = "config.yml";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub comm_port: String,
    pub base_layer: u8,
    pub mappings: Vec<AppConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            comm_port: String::from("COM5"),
            base_layer: 1,
            mappings: vec![],
        }
    }
}

impl Config {
    pub fn load() -> Config {
        let file = File::open(CONFIG_PATH);
        let config = match file {
            Ok(mut f) => {
                let mut contents = String::new();
                match f.read_to_string(&mut contents) {
                    Ok(_) => match serde_yaml::from_str(&contents) {
                        Ok(data) => {
                            debug!("Config loaded");
                            data
                        }
                        Err(e) => {
                            error!("Error parsing config file: {:?}", e);
                            Config::default()
                        }
                    },
                    Err(e) => {
                        error!("Error reading config file: {:?}", e);
                        Config::default()
                    }
                }
            }
            Err(_) => {
                warn!("No config file found, creating with default config");
                let default_config = Config::default();
                match File::create(CONFIG_PATH) {
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

    pub fn add(&mut self) {
        self.mappings.push(AppConfig::default());
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.mappings.len() {
            self.mappings.remove(index);
        }
    }

    pub fn save(&self) {
        match File::create(CONFIG_PATH) {
            Ok(file) => {
                let mut writer = io::BufWriter::new(file);
                if let Err(e) = serde_yaml::to_writer(&mut writer, &self) {
                    error!("Failed to serialize config: {:?}", e);
                }
                if let Err(e) = writer.flush() {
                    error!("Failed to write config to file: {:?}", e);
                }
            }
            Err(e) => {
                error!("Failed to create config file: {:?}", e);
            }
        }
    }

    pub fn check_exe_name(&self, app_details: &AppDetails) -> Option<u8> {
        app_details
            .exe_name
            .as_ref()
            .and_then(|app_exe_name| self.match_property(app_exe_name, |mapping| &mapping.exe_name))
    }

    pub fn check_window_title(&self, app_details: &AppDetails) -> Option<u8> {
        app_details
            .window_title
            .as_ref()
            .and_then(|app_window_title| {
                self.match_property(app_window_title, |mapping| &mapping.window_title)
            })
    }

    fn match_property<T, F>(&self, app_property: T, mapping_property: F) -> Option<u8>
    where
        T: AsRef<str>,
        F: Fn(&AppConfig) -> &Option<String>,
    {
        self.mappings.iter().find_map(|mapping| {
            match (
                app_property.as_ref().to_lowercase(),
                mapping_property(mapping),
            ) {
                (app_prop, Some(map_prop)) if map_prop.to_lowercase().contains(&app_prop) => {
                    Some(mapping.layer)
                }
                _ => None,
            }
        })
    }
}
