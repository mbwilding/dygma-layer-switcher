use crate::TITLE;
use anyhow::Result;
use single_instance::SingleInstance;
use tracing::error;

pub fn check() -> Result<()> {
    let instance = SingleInstance::new(TITLE)?;

    if !instance.is_single() {
        error!("Another instance of Dygma Layer Switcher is already running");
        std::process::exit(1);
    }

    Ok(())
}
