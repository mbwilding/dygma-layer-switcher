// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use eframe::*;

mod app;
mod helpers;
mod log;
mod single;
mod structs;
mod templates;

pub fn main() -> Result<()> {
    single::check()?;

    let options = NativeOptions {
        // viewport: Default::default(),
        vsync: true,
        follow_system_theme: true,
        default_theme: Theme::Dark,
        // run_and_return: false,
        // event_loop_builder: None,
        centered: true,
        persist_window: true,
        ..Default::default()
    };

    run_native(
        "Dygma Layer Switcher",
        options,
        Box::new(|cc| {
            let app = app::DygmaLayerSwitcher::new(cc);
            log::init(app.logging);
            Box::new(app)
        }),
    )?;

    Ok(())
}
