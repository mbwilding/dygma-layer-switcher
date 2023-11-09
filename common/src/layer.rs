use crate::app::AppDetails;
use crate::config::Config;
use crate::serial;
use anyhow::Result;
use lazy_static::lazy_static;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info};

lazy_static! {
    static ref LAYER_CACHE: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
}

pub fn process(app_details: &AppDetails) -> Result<()> {
    debug!("{:#?}", app_details);

    let config = Config::load(); // TODO: Don't call this every time, ideally we'd want to use the `notify' crate

    let mut layer_current = match LAYER_CACHE.lock() {
        Ok(guard) => guard,
        Err(e) => {
            error!("Failed to acquire lock on LAYER_CACHE: {:#?}", e);
            return Ok(());
        }
    };

    let layer_desired = config
        // Check exe name first
        .check_exe_name(app_details)
        // Check window title second
        .or_else(|| config.check_window_title(app_details))
        // Returns to base layer automatically if no matches
        .unwrap_or(config.base_layer);

    // Layer hasn't changed, no need to do anything (Assumes user has set the layer manually)
    if layer_desired == *layer_current {
        return Ok(());
    }

    if layer_change(&config, layer_desired)? {
        // Set the cache to the desired layer
        *layer_current = layer_desired;
    }

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
