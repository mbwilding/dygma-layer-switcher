#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use tracing::error;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(if cfg!(debug_assertions) {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .with_ansi(true)
        .init();

    if cfg!(target_os = "windows") {
        windows::init::start()?;
    } else {
        error!("This is currently an unsupported OS")
    }

    Ok(())
}
