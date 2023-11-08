use crate::app::App;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub comm_port: String,
    pub base_layer: u8,
    pub mappings: BTreeMap<String, u8>,
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
                            info!("Error parsing config file: {:?}", e);
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
                info!("No config file found, creating with default config");
                let default_config = Config::default();
                match File::create(config_path) {
                    Ok(mut file) => {
                        let yaml = serde_yaml::to_string(&default_config)
                            .expect("Failed to serialize default config");
                        file.write_all(yaml.as_bytes())
                            .expect("Failed to write default config to file");
                    }
                    Err(e) => {
                        info!("Failed to create config file: {:?}", e);
                    }
                }

                default_config
            }
        };

        config
    }

    pub fn check_exe_name(&self, app: &App) -> Option<u8> {
        self.mappings.get(&app.exe_name).copied()
    }

    pub fn check_window_title(&self, app: &App) -> Option<u8> {
        self.mappings
            .iter()
            .find(|(key, _)| key.contains(app.window_title.as_str()))
            .map(|(_, &value)| value)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            comm_port: String::from("COM5"),
            base_layer: 0,
            mappings: BTreeMap::new(),
        }
    }
}
