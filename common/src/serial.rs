use crate::app::App;
use crate::config::Config;
use lazy_static::lazy_static;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::{debug, error};

type SafeLayer = Arc<Mutex<u8>>;

lazy_static! {
    static ref LAYER_PREV: SafeLayer = Arc::new(Mutex::new(255));
}

pub fn process(app: &App) {
    let config = Config::get_config(); // TODO Don't call this every time

    debug!("App details: {:#?}", app);

    let mut layer_previous_guard = match LAYER_PREV.lock() {
        Ok(guard) => guard,
        Err(e) => {
            error!("Failed to acquire lock on LAYER_PREV: {:#?}", e);
            return;
        }
    };

    let layer_resolved = config
        .check_exe_name(app)
        .or_else(|| config.check_window_title(app))
        .unwrap_or(config.base_layer);

    if layer_resolved == *layer_previous_guard {
        return;
    }

    if layer_resolved == *layer_previous_guard {
        return;
    }

    if *layer_previous_guard != 255 {
        debug!(
            "Attempting layer change from {} to {}",
            *layer_previous_guard, &layer_resolved
        );
    } else {
        debug!("Attempting layer change to {}", &layer_resolved);
    }

    let mut port = match serialport::new(config.comm_port.clone(), 9_600)
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(std::time::Duration::from_millis(10))
        .open()
    {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Failed to open serial port: {} ({:#?})",
                &config.comm_port, e
            );
            return;
        }
    };

    let command = format!("layer.moveTo {:?}\n", &layer_resolved);
    if port.write_all(command.as_bytes()).is_ok() {
        debug!("Changed layer to {}", layer_resolved);
    } else {
        error!("Failed to write to serial port: {:?}", &config.comm_port);
        return;
    }

    *layer_previous_guard = layer_resolved;
}
