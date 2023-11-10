// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::init::{app_init, log_init};

#[cfg(windows)]
use windows::service::my_service_main;

#[cfg(windows)]
windows_service::define_windows_service!(ffi_service_main, my_service_main);

fn main() -> anyhow::Result<()> {
    log_init();
    app_init()?;

    #[cfg(windows)]
    windows::init::start()?;

    #[cfg(not(windows))]
    tracing::error!("Platform not yet supported");

    Ok(())
}
