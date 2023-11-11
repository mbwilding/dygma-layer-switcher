use crate::app::AppDetails;
use crate::config::Config;
use crate::serial;
use anyhow::Result;
use std::io::Write;
use tracing::{debug, error, info};

pub fn process(app_details: &AppDetails) -> Result<()> {
    debug!("{:#?}", app_details);

    let config = Config::load(); // TODO: Don't call this every time, ideally we'd want to use the `notify' crate

    let layer_desired = config
        // Check exe name first
        .check_exe_name(app_details)
        // Check window title second
        .or(config.check_window_title(app_details))
        // Fallback to base layer, or default if base layer is also None
        .or(config.base_layer)
        .unwrap_or(1); // Default layer value if all else fails

    let _ = layer_change(&config, layer_desired)?;

    Ok(())
}

fn layer_change(config: &Config, layer: u8) -> Result<bool> {
    let mut port = match serial::configure(config) {
        Ok(x) => x,
        Err(_) => return Ok(false),
    };

    let command = format!("layer.moveTo {:?}\n", &layer - 1);

    if port.write_all(command.as_bytes()).is_ok() {
        info!("Layer: {}", layer);
        Ok(true)
    } else {
        error!("Failed to write to serial port: {:?}", &config.comm_port);
        Ok(false)
    }
}
