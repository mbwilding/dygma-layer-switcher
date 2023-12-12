use crate::app::{AppConfig, AppDetails};
use crate::serial;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use tracing::{debug, error, warn};

const CONFIG_PATH: &str = "config.yml";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub logging: Option<bool>,
    pub comm_port: Option<String>,
    pub base_layer: Option<u8>,
    pub mappings: Option<Vec<AppConfig>>,
}

impl Default for Config {
    fn default() -> Self {
        let ports = serial::detect_ports();

        // NOTE: There are different port types, i.e. bluetooth, usb, etc.
        // Could be useful for auto-detecting the correct port
        let comm_port = match ports {
            Ok(ports) => match ports.first() {
                Some(port) => port.port_name.clone(),
                None => {
                    warn!("No serial ports detected, defaulting to COM5");
                    String::from("COM4")
                }
            },
            Err(e) => {
                error!("Failed to detect serial ports: {:?}", e);
                String::from("COM4")
            }
        };

        Config {
            logging: Some(false),
            comm_port: Some(comm_port),
            base_layer: Some(1),
            mappings: Some(vec![]),
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
                    Ok(mut file) => match serde_yaml::to_string(&default_config) {
                        Ok(yaml) => {
                            if let Err(e) = file.write_all(yaml.as_bytes()) {
                                error!("Failed to write default config to file: {:?}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to serialize default config: {:?}", e);
                        }
                    },
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
        if let Some(mappings) = self.mappings.as_mut() {
            mappings.push(AppConfig::default());
        } else {
            error!("Mappings not initialized");
        }
    }

    pub fn remove(&mut self, index: usize) {
        if let Some(mappings) = self.mappings.as_mut() {
            if index < mappings.len() {
                mappings.remove(index);
            }
        } else {
            error!("Mappings not initialized");
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
        let app_prop = app_property.as_ref().to_lowercase();

        self.mappings.as_ref()?.iter().find_map(|mapping| {
            mapping_property(mapping)
                .as_ref()?
                .to_lowercase()
                .contains(&app_prop)
                .then_some(mapping.layer)
        })
    }
}
