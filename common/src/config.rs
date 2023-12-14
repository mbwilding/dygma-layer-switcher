use crate::app::{AppConfig, AppDetails, Parent};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use sysinfo::{Process, ProcessExt, System, SystemExt};
use tracing::{debug, error, trace, warn};

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
        let focus = dygma_focus::Focus::default();

        let device = focus.find_first().unwrap_or_else(|e| {
            error!(
                "Connect a Dygma keyboard and restart the application: {:?}",
                e
            );
            std::process::exit(1);
        });

        Config {
            logging: Some(false),
            comm_port: Some(device.port),
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
                            trace!("Config loaded");
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

    pub fn check_process(&self, app_details: &AppDetails) -> Option<u8> {
        app_details
            .process
            .as_ref()
            .and_then(|x| self.match_property_opt_string(x, |mapping| &mapping.process))
    }

    pub fn check_window(&self, app_details: &AppDetails) -> Option<u8> {
        app_details
            .window
            .as_ref()
            .and_then(|x| self.match_property_opt_string(x, |mapping| &mapping.window))
    }

    pub fn check_parent(&self, app_details: &AppDetails) -> Option<u8> {
        if let Some(process) = &app_details.process {
            if self.is_process_excluded(process) {
                debug!("Process was excluded from parent process: {:?}", process);
                return None;
            }

            let mut sys = System::new();
            sys.refresh_processes();

            if let Some((_, proc)) = sys
                .processes()
                .iter()
                .find(|(_, proc)| proc.name().to_lowercase().contains(&process.to_lowercase()))
            {
                return self.check_parent_recursively(proc, &sys, |mapping| &mapping.parent, 1);
            }
        }

        None
    }

    fn is_process_excluded(&self, process: &str) -> bool {
        self.mappings.as_ref().map_or(false, |mappings| {
            mappings.iter().any(|mapping| {
                if let Some(parent) = &mapping.parent {
                    if let Some(excludes) = &parent.excludes {
                        return excludes.iter().any(|excluded_process| {
                            excluded_process
                                .to_lowercase()
                                .contains(&process.to_lowercase())
                        });
                    }
                }

                false
            })
        })
    }

    fn check_parent_recursively<F>(
        &self,
        proc: &Process,
        sys: &System,
        mapping_property: F,
        level: usize,
    ) -> Option<u8>
    where
        F: Fn(&AppConfig) -> &Option<Parent> + Copy,
    {
        // Recursive call with the parent process
        if let Some(parent_pid) = proc.parent() {
            if let Some(parent_proc) = sys.processes().get(&parent_pid) {
                debug!("Parent: {:?}, Level: {}", parent_proc.name(), level);
                if let Some(layer) =
                    self.match_property_opt_parent(parent_proc.name(), mapping_property)
                {
                    return Some(layer);
                }
                return self.check_parent_recursively(
                    parent_proc,
                    sys,
                    mapping_property,
                    level + 1,
                );
            }
        }

        None
    }

    fn match_property_opt_parent<T, F>(&self, app_property: T, mapping_property: F) -> Option<u8>
    where
        T: AsRef<str>,
        F: Fn(&AppConfig) -> &Option<Parent>,
    {
        let app_prop = app_property.as_ref().to_lowercase();

        self.mappings.as_ref()?.iter().find_map(|mapping| {
            mapping_property(mapping)
                .as_ref()?
                .process
                .as_ref()?
                .to_lowercase()
                .contains(&app_prop)
                .then_some(mapping.layer)
        })
    }

    fn match_property_opt_string<T, F>(&self, app_property: T, mapping_property: F) -> Option<u8>
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
