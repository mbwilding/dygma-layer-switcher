use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

pub fn init(logging: bool) {
    let console_filter = EnvFilter::new("debug")
        .add_directive("dygma-layer-switcher=trace".parse().unwrap())
        .add_directive("egui=warn".parse().unwrap())
        .add_directive("eframe=warn".parse().unwrap())
        .add_directive("wgpu=warn".parse().unwrap());

    let console_layer = fmt::layer()
        .with_ansi(true)
        .without_time()
        .compact()
        .with_filter(console_filter);

    let subscriber_builder = tracing_subscriber::registry();

    if logging {
        let file_filter = EnvFilter::new("trace")
            .add_directive("dygma-layer-switcher=trace".parse().unwrap())
            .add_directive("eframe=warn".parse().unwrap())
            .add_directive("egui=warn".parse().unwrap())
            .add_directive("wgpu=warn".parse().unwrap());

        subscriber_builder
            .with(console_layer)
            .with(
                fmt::layer()
                    .with_ansi(false)
                    .with_writer(tracing_appender::rolling::daily("logs", "dsl"))
                    .with_filter(file_filter),
            )
            .init();
    } else {
        subscriber_builder.with(console_layer).init();
    }
}
