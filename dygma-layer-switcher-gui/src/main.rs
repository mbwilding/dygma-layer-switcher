// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate core;

mod app;
mod helpers;
mod log;
mod structs;
mod templates;

pub fn main() {
    let options = eframe::NativeOptions {
        // viewport: Default::default(),
        vsync: true,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        // run_and_return: false,
        // event_loop_builder: None,
        centered: true,
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Dygma Layer Switcher",
        options,
        Box::new(|cc| {
            let app = app::DygmaLayerSwitcher::new(cc);
            log::init(app.logging);
            Box::new(app)
        }),
    )
    .unwrap();
}
