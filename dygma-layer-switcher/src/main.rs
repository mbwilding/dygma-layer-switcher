// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use single_instance::SingleInstance;
use tracing::error;

pub mod tray;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(if cfg!(debug_assertions) {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .with_ansi(true)
        .init();

    let instance = SingleInstance::new("dygma-layer-switcher")?;

    if !instance.is_single() {
        error!("Another instance of Dygma Layer Switcher is already running");
        std::process::exit(1);
    }

    tray::load()?;

    #[cfg(target_os = "windows")]
    {
        windows::init::start()?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        error!("This is currently an unsupported OS")
    }

    Ok(())
}
