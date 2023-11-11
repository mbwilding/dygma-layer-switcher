use crate::config::Config;
use anyhow::Result;
use single_instance::SingleInstance;
use tracing::error;

#[allow(unused_variables)]
pub fn log_init(config: &Config) {
    let tracing = tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG);

    #[cfg(debug_assertions)]
    {
        tracing.with_ansi(true).init();
    }

    #[cfg(not(debug_assertions))]
    {
        let tracing = tracing.with_ansi(false);

        if let Some(logging) = config.logging {
            if logging {
                let file_appender = tracing_appender::rolling::daily("logs", "dsl.log");
                tracing.with_writer(file_appender).init();
            }
        } else {
            tracing.init();
        }
    }
}

pub fn single_check() -> Result<()> {
    let instance = SingleInstance::new("dygma-layer-switcher")?;

    if !instance.is_single() {
        error!("Another instance of Dygma Layer Switcher is already running");
        std::process::exit(1);
    }

    Ok(())
}
