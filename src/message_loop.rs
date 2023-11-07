use tracing::{debug, error, info};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Accessibility::{HWINEVENTHOOK, UnhookWinEvent};
use serialport::prelude::*;
use crate::window_data::{get_exe_name, get_window_title};
use crate::window::Window;
use anyhow::{anyhow, Result};

pub struct EventHook(pub HWINEVENTHOOK);

use serde::Deserialize;
use std::collections::HashMap;
use std::{env, fs};
use std::path::PathBuf;
use serde_yaml;

impl Drop for EventHook {
    /// destcructor for the EventHook
    fn drop(&mut self) {
        unsafe {
            UnhookWinEvent(self.0);
        }
        debug!("Unhooked");
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
struct ConfigDictionary {
    // Assuming the YAML structure contains a dictionary
    mappings: HashMap<String, String>,
    port: String,
    baselayer: String,
}

fn load_yaml_dictionary(config_path: &PathBuf) -> Result<ConfigDictionary> {
    let file_path: PathBuf;
    if !config_path.exists() {
        file_path = PathBuf::from("config.yaml");
    } else {
        file_path = (**config_path).to_path_buf();
    }
    let file_content= fs::read_to_string(file_path)?;
    let data: ConfigDictionary = serde_yaml::from_str(&file_content)?;
    info!("{:#?}", data);
    Ok(data)
}

pub unsafe extern "system" fn get_focused_window_details(
    _h_win_event_hook: HWINEVENTHOOK,
    _event: u32,
    window_handle: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    if window_handle.0 == 0 {
        return; // return because thereis no return value
    }

    debug!("Window handle: {:?}", window_handle.0);

    let window_title = get_window_title(window_handle);

    // matches the exe name, if it is a string, set the value, if not not set empty string
    let exe_name = match get_exe_name(window_handle) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to get the execuatable name: {:?}", e);
            String::new()
        }
    };

    let window_details = Window {
        window_title,
        exe_name,
    };

    // load config
    // get current directory
    let env_path = env::current_exe().expect("Failed to get current executable");
    let env_dir = env_path.parent().expect("Failed to get env directory.");
    let config_path = env_dir.join("config.yaml");
    let config_dictionary = load_yaml_dictionary(&config_path);

    let config = config_dictionary.unwrap();
    let port_id = config.port;
    info!("Port: {}", port_id);
    let key: &String = &window_details.exe_name;
    let mut layer = config.baselayer.clone();
    // get key from dictionary
    for (key, value) in &config.mappings {
        // println!("{:?} {:?}", key, value);
        if window_details.exe_name.contains(key)  {
            layer = value.to_string();
            break;
        }
    }

    // if layer not in exe names, check for window names
    if layer == config.baselayer {
        for (key, value) in &config.mappings {
            if window_details.window_title.contains(key) {
                layer = value.to_string();
                break;
            }
        }
    }
    // let layer: &String = config.mappings.get(key).unwrap();
    if !port_id.is_empty() {
        let settings = SerialPortSettings {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: std::time::Duration::from_millis(10), // Adjust timeout as needed
        };

        // define the command, \n to confirm the input
        let command = format!("layer.moveTo {:?}\n", layer);

        match serialport::open_with_settings(&port_id, &settings) {
            Ok(mut port) => {

                // Write the command to the serial port
                if let Err(err) = port.write_all(command.as_bytes()) {
                    error!("Failed to write to the serial port: {}", err);
                } else {
                    info!("Sent: {}", command);
                }
            }
            Err(err) => {
                error!("Failed to open the serial port: {}", err);
            }
        }
        // info!("Sending layer-switch command: {:?}", data_to_send);
    } else {
        error!("Failed to switch layers!")
    }

    info!("Window details: {:#?}", window_details)
}
