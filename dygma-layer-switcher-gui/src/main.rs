extern crate core;

mod app;
mod structs;
mod templates;

pub fn main() {
    eframe::run_native(
        "Dygma Layer Switcher",
        eframe::NativeOptions {
            // viewport: Default::default(),
            vsync: true,
            follow_system_theme: true,
            default_theme: eframe::Theme::Dark,
            // run_and_return: false,
            // event_loop_builder: None,
            centered: true,
            persist_window: true,
            ..Default::default()
        },
        Box::new(|cc| Box::new(app::DygmaLayerSwitcher::new(cc))),
    )
    .unwrap();
}
