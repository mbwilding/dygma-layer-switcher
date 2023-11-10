use crate::tray;
use anyhow::Result;
use single_instance::SingleInstance;
use tracing::error;

pub fn log_init() {
    tracing_subscriber::fmt()
        .with_max_level(if cfg!(debug_assertions) {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .with_ansi(true)
        .init();
}

pub fn app_init() -> Result<()> {
    let instance = SingleInstance::new("dygma-layer-switcher")?;

    if !instance.is_single() {
        error!("Another instance of Dygma Layer Switcher is already running");
        std::process::exit(1);
    }

    tray::load()?;

    Ok(())
}
