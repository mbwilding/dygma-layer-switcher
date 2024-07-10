// hide console window on Windows
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use common::verbiage;
use eframe::egui::ViewportBuilder;
use eframe::*;
use std::sync::Arc;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod app;
mod helpers;
mod icon;
mod single;
mod templates;

pub fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::OFF.into())
                .from_env_lossy(),
        )
        .with_target(false)
        .without_time()
        .compact()
        .init();

    single::check()?;

    run_native(
        verbiage::APP_NAME,
        NativeOptions {
            default_theme: Theme::Dark,
            // follow_system_theme: true,
            persist_window: true,
            centered: false,
            vsync: true,
            viewport: ViewportBuilder::default()
                .with_inner_size((400.0, 320.0))
                .with_resizable(true)
                .with_close_button(true)
                .with_minimize_button(true)
                .with_maximize_button(true)
                .with_icon(Arc::new(icon::load_icon(include_bytes!(
                    "../../../assets/icons/icon.ico"
                )))),
            ..Default::default()
        },
        Box::new(move |cc| {
            let mut app = app::DygmaLayerSwitcher::new(cc);
            app.configuration_changed = true;

            #[cfg(target_os = "windows")]
            windows::windows::start();

            #[cfg(target_os = "macos")]
            macintosh::macintosh::start();

            Ok(Box::new(app))
        }),
    )?;

    Ok(())
}
