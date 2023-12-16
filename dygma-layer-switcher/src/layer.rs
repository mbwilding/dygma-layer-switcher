use crate::structs::AppDetails;
use tracing::{debug, error, info};

pub fn process(app_details: &AppDetails) {
    debug!("{:?}", app_details);

    // TODO
    // let config = Config::load();

    // let layer = config
    //     .check_window(app_details)
    //     .or_else(|| config.check_process(app_details))
    //     .or_else(|| config.check_parent(app_details))
    //     .unwrap_or_else(|| config.base_layer.unwrap_or_default());

    // layer_change(&config, layer);
}

fn layer_change(layer: u8) {
    // if let Some(port) = &config.comm_port {
    //     let mut focus = dygma_focus::Focus::default();
    //     if focus.open_via_port(port).is_ok() {
    //         if focus.layer_move_to(layer - 1).is_ok() {
    //             info!("Changed layer: {}", layer);
    //         } else {
    //             error!("Failed to write to serial port: {:?}", &config.comm_port);
    //         }
    //     } else {
    //         error!("Failed to open serial port: {:?}", &config.comm_port);
    //     }
    // } else {
    //     error!("No serial port specified");
    // }
}
