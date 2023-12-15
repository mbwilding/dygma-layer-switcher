// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod log;

use common::config::Config;
use common::single;
use tracing::error;

fn main() -> anyhow::Result<()> {
    let config = Config::load();
    log::init(&config);

    #[cfg(not(windows))]
    error!("Platform not yet supported");

    single::check()?;

    #[cfg(windows)]
    {
        windows::init::start();

        windows::tray::load().unwrap_or_else(|e| {
            error!("Failed to load tray: {}", e);
            std::process::exit(1);
        });
    }

    Ok(())
}
