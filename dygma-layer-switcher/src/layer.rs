use crate::app::DygmaLayerSwitcher;
use crate::structs::AppDetails;
use tracing::{error, info};

pub fn process(config: &DygmaLayerSwitcher, app_details: &AppDetails) {
    let layer = check_window(app_details)
        .or_else(|| check_process(app_details))
        .or_else(|| check_parent(app_details))
        .unwrap_or(config.base_layer);

    layer_change(config, layer);
}

fn layer_change(config: &DygmaLayerSwitcher, layer: u8) {
    let mut focus = dygma_focus::Focus::default();
    if focus.open_via_port(&config.port).is_ok() {
        if focus.layer_move_to(layer - 1).is_ok() {
            info!("Changed layer: {}", layer);
        } else {
            error!("Failed to write to serial port: {:?}", &config.port);
        }
    } else {
        error!("Failed to open serial port: {:?}", &config.port);
    }
}

fn check_window(app_details: &AppDetails) -> Option<u8> {
    None
}

fn check_process(app_details: &AppDetails) -> Option<u8> {
    None
}

fn check_parent(app_details: &AppDetails) -> Option<u8> {
    None
}
