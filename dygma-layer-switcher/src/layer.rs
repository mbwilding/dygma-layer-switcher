use crate::app::{DygmaLayerSwitcher, SHARED_STATE};
use crate::structs::{AppDetails, Mode};
use sysinfo::{ProcessExt, System, SystemExt};
use tracing::{error, info};

pub fn process(app_details: &AppDetails) {
    let config = SHARED_STATE.lock().unwrap();

    let layer = check_window(&config, app_details)
        .or_else(|| check_process(&config, app_details))
        .or_else(|| check_parent(&config, app_details))
        .unwrap_or(config.base_layer - 1);

    layer_change(&config, layer);

    drop(config);
}

fn layer_change(config: &DygmaLayerSwitcher, layer: u8) {
    let mut focus = dygma_focus::Focus::default();
    match focus.open_via_port(&config.port) {
        Ok(_) => {
            if let Err(e) = focus.layer_move_to(layer) {
                error!("Failed to write to serial port '{}': {:?}", &config.port, e);
            } else {
                info!("Changed layer: {}", config.mappings[&layer].name);
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
                if app.is_enabled
                    && window
                        .name
                        .to_lowercase()
                        .contains(&app_details.window.to_lowercase())
                {
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
                if app.is_enabled
                    && process.name.to_lowercase() == app_details.process.to_lowercase()
                {
                    return Some(layer_number);
                }
            }
        }
    }

    None
}

fn check_parent(config: &DygmaLayerSwitcher, app_details: &AppDetails) -> Option<u8> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Find the specified process.
    let specified_process = sys
        .processes()
        .values()
        .find(|p| p.name() == app_details.process);

    if let Some(proc) = specified_process {
        // Check each parent process of the specified process.
        let mut current_proc = Some(proc);
        while let Some(proc) = current_proc {
            for (&layer_number, layer) in &config.mappings {
                for app in &layer.apps {
                    if let Mode::Parent(ref parent) = app.mode {
                        if app.is_enabled && parent.name == proc.name() {
                            // Check if the process is disabled or is not excluded.
                            let is_excluded = parent.excludes.iter().any(|exclude| {
                                exclude.is_enabled
                                    && exclude.name.to_lowercase()
                                        == app_details.process.to_lowercase()
                            });

                            if !is_excluded {
                                return Some(layer_number);
                            }
                        }
                    }
                }
            }

            // Move to the parent process in the next iteration.
            current_proc = proc.parent().and_then(|pid| sys.process(pid));
        }
    }

    None
}
