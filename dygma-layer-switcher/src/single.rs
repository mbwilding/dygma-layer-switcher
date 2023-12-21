use crate::verbiage;
use anyhow::Result;
use log::error;
use single_instance::SingleInstance;

pub fn check() -> Result<()> {
    let instance = SingleInstance::new(verbiage::APP_NAME)?;

    if !instance.is_single() {
        error!("Another instance of Dygma Layer Switcher is already running");
        std::process::exit(1);
    }

    Ok(())
}
