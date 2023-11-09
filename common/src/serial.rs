use crate::app::App;
use crate::config::Config;
use lazy_static::lazy_static;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info};

lazy_static! {
    static ref LAYER_CACHE: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
}

pub fn process(app: &App) {
    let config = Config::load(); // TODO: Don't call this every time, ideally we'd want to use the `notify' crate

    debug!("{:#?}", app);

    let mut layer_current = match LAYER_CACHE.lock() {
        Ok(guard) => guard,
        Err(e) => {
            error!("Failed to acquire lock on LAYER_CACHE: {:#?}", e);
            return;
        }
    };

    let layer_desired = config
        .check_exe_name(app)
        .or_else(|| config.check_window_title(app))
        .unwrap_or(config.base_layer);

    if layer_desired == *layer_current {
        return;
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

    let command = format!("layer.moveTo {:?}\n", &layer_desired - 1);
    if port.write_all(command.as_bytes()).is_ok() {
        info!("Layer: {}", layer_desired);
    } else {
        error!("Failed to write to serial port: {:?}", &config.comm_port);
        return;
    }

    *layer_current = layer_desired;
}
