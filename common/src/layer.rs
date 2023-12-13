use crate::app::AppDetails;
use crate::config::Config;
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
    if let Some(port) = &config.comm_port {
        let mut focus = dygma_focus::Focus::default();
        if focus.open_via_port(port).is_ok() {
            if focus.layer_move_to(layer - 1).is_ok() {
                info!("Changed layer: {}", layer);
            } else {
                error!("Failed to write to serial port: {:?}", &config.comm_port);
            }
        } else {
            error!("Failed to open serial port: {:?}", &config.comm_port);
        }
    } else {
        error!("No serial port specified");
    }
}
