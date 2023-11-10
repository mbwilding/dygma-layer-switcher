use anyhow::Result;
use single_instance::SingleInstance;
use tracing::error;

pub fn log_init() {
    let tracing = tracing_subscriber::fmt();

    #[cfg(debug_assertions)]
    {
        tracing
            .with_max_level(tracing::Level::DEBUG)
            .with_ansi(true)
            .init();
    }

    #[cfg(not(debug_assertions))]
    {
        let file_appender = tracing_appender::rolling::daily("logs", "dsl.log");
        tracing
            .with_max_level(tracing::Level::INFO)
            .with_ansi(false)
            .with_writer(file_appender)
            .init();
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
