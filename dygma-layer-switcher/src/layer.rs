use crate::app::DygmaLayerSwitcher;
use crate::structs::{AppDetails, Mode};
use tracing::{error, info};

pub fn process(config: &DygmaLayerSwitcher, app_details: &AppDetails) {
    let layer = check_window(config, app_details)
        .or_else(|| check_process(config, app_details))
        .or_else(|| check_parent(config, app_details))
        .unwrap_or(config.base_layer);

    layer_change(config, layer);
}

fn layer_change(config: &DygmaLayerSwitcher, layer: u8) {
    let mut focus = dygma_focus::Focus::default();
    match focus.open_via_port(&config.port) {
        Ok(_) => {
            if let Err(e) = focus.layer_move_to(layer - 1) {
                error!("Failed to write to serial port '{}': {:?}", &config.port, e);
            } else {
                info!("Changed layer: {}", layer);
            }
        }
        Err(e) => {
            error!("Failed to open serial port '{}': {:?}", &config.port, e);
        }
    }
}

fn check_window(config: &DygmaLayerSwitcher, app_details: &AppDetails) -> Option<u8> {
    for (&layer_number, layer) in &config.mappings {
        for app in &layer.apps {
            if let Mode::Window(ref window) = app.mode {
                if app.is_enabled && window.name == app_details.window {
                    return Some(layer_number);
                }
            }
        }
    }

    None
}

fn check_process(config: &DygmaLayerSwitcher, app_details: &AppDetails) -> Option<u8> {
    for (&layer_number, layer) in &config.mappings {
        for app in &layer.apps {
            if let Mode::Process(ref process) = app.mode {
                if app.is_enabled && process.name == app_details.process {
                    return Some(layer_number);
                }
            }
        }
    }

    None
}

fn check_parent(_config: &DygmaLayerSwitcher, _app_details: &AppDetails) -> Option<u8> {
    None
}
