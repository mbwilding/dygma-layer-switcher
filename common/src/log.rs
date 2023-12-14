use crate::config::Config;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

pub fn init(config: &Config) {
    if let Some(logging) = config.logging {
        let console_layer = fmt::layer()
            .with_ansi(true)
            .without_time()
            .compact()
            .with_filter(LevelFilter::DEBUG);

        let subscriber_builder = tracing_subscriber::registry();

        if logging {
            subscriber_builder
                .with(console_layer)
                .with(
                    fmt::layer()
                        .with_ansi(false)
                        .with_writer(tracing_appender::rolling::daily("logs", "dsl"))
                        .with_filter(LevelFilter::TRACE),
                )
                .init();
        } else {
            subscriber_builder.with(console_layer).init();
        }
    }
}
