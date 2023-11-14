// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::config::Config;
use common::{log::init, single, tray};
use tracing::error;

#[cfg(windows)]
use windows::service::windows_service_main;

#[cfg(windows)]
windows_service::define_windows_service!(ffi_service_main, windows_service_main);

fn main() -> anyhow::Result<()> {
    #[cfg(windows)]
    // This  will fail fast if it is not being launched as a service and execution will continue
    if windows_service::service_dispatcher::start("Dygma Layer Switcher", ffi_service_main).is_ok()
    {
        // Service has finished, we've already ran the logic below internally, so we can just return
        return Ok(());
    };

    let config = Config::load();
    init(&config);

    #[cfg(not(windows))]
    error!("Platform not yet supported");

    single::check()?;

    #[cfg(windows)]
    windows::init::start();

    tray::load().unwrap_or_else(|e| {
        error!("Failed to load tray: {}", e);
        std::process::exit(1);
    });

    Ok(())
}
