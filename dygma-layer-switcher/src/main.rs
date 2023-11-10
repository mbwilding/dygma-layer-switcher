// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::init::{log_init, single_check};
use common::tray;

#[cfg(windows)]
use windows::service::windows_service_main;

#[cfg(windows)]
windows_service::define_windows_service!(ffi_service_main, windows_service_main);

fn main() -> anyhow::Result<()> {
    #[cfg(windows)]
    if windows_service::service_dispatcher::start("Dygma Layer Switcher", ffi_service_main).is_ok()
    {
        return Ok(());
    };

    log_init();
    single_check()?;
    tray::load()?;

    #[cfg(windows)]
    windows::init::start()?;

    #[cfg(not(windows))]
    tracing::error!("Platform not yet supported");

    Ok(())
}
