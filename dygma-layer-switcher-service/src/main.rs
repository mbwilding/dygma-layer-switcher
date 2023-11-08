#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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
