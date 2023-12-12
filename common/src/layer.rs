use crate::app::AppDetails;
use crate::config::Config;
use crate::serial;
use std::io::Write;
use tracing::{debug, error, info};

pub fn process(app_details: &AppDetails) {
    debug!("{:?}", app_details);

    let config = Config::load();

    let layer_desired = config
        .check_window(app_details)
        .or(config.check_process(app_details))
        .or_else(|| {
            if config.mappings.iter().flatten().any(|x| x.parent.is_some()) {
                config.check_parent(app_details)
            } else {
                None
            }
        })
        .unwrap_or(config.base_layer.unwrap());

    layer_change(&config, layer_desired);
}

fn layer_change(config: &Config, layer: u8) {
    if let Ok(mut port) = serial::configure(config) {
        let command = format!("layer.moveTo {:?}\n", &layer - 1);

        if port.write_all(command.as_bytes()).is_ok() {
            info!("Layer: {}", layer);
        } else {
            error!("Failed to write to serial port: {:?}", &config.comm_port);
        }
    };
}
