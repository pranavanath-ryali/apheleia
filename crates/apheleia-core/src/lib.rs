pub(crate) mod cell;
pub mod buffer;
pub mod terminal;
pub mod style;
pub mod rich_string;

use std::error::Error;

use tracing::{info};
use tracing_subscriber::{EnvFilter, fmt::{self, FormatEvent, FormatFields, format::Writer}, prelude::*, registry::LookupSpan};

struct CustomFormatter;

impl<S, N> FormatEvent<S, N> for CustomFormatter
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();

        // [LOG_TYPE]
        write!(writer, "[{}] ", metadata.level())?;

        // [TIME]
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        write!(writer, "[{}] ", now)?;

        // [TARGET / MODULE]
        write!(writer, "[{}] ", metadata.target())?;

        // Message
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

pub fn setup_logger() -> Result<tracing_appender::non_blocking::WorkerGuard, Box<dyn Error>> {
    // Non-blocking file writer to app.log
    let file_appender = tracing_appender::rolling::never(".", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Filter levels for the file (defaults to DEBUG; can be overridden via RUST_LOG)
    let file_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug"));

    // File Layer ONLY
    let file_layer = fmt::layer()
        .event_format(CustomFormatter)
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(file_filter);

    // Register only the file layer
    tracing_subscriber::registry()
        .with(file_layer)
        .init();

    info!(target: "CORE", "Log started");

    Ok(guard)
}
