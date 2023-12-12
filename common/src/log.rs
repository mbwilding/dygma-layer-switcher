use crate::config::Config;

pub fn init(config: &Config) {
    let tracing = tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG);

    if let Some(logging) = config.logging {
        if logging {
            let file_appender = tracing_appender::rolling::daily("logs", "dsl");
            tracing.with_ansi(false).with_writer(file_appender).init();
        } else {
            tracing.with_ansi(true).init();
        }
    } else {
        tracing.with_ansi(true).init();
    }
}
