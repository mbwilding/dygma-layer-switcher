use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

pub fn init(logging: bool) {
    let base_filter = EnvFilter::new("info")
        .add_directive("dygma-layer-switcher-gui=trace".parse().unwrap())
        .add_directive("egui=warn".parse().unwrap())
        .add_directive("wgpu=warn".parse().unwrap());

    let console_layer = fmt::layer()
        .with_ansi(true)
        .without_time()
        .compact()
        .with_filter(base_filter);

    let subscriber_builder = tracing_subscriber::registry();

    if logging {
        subscriber_builder
            .with(console_layer)
            .with(
                fmt::layer()
                    .with_ansi(false)
                    .with_writer(tracing_appender::rolling::daily("logs", "dsl"))
                    .with_filter(LevelFilter::DEBUG),
            )
            .init();
    } else {
        subscriber_builder.with(console_layer).init();
    }
}
