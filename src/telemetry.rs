use directories::ProjectDirs;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter, fmt, prelude::*, EnvFilter};

pub fn setup_log(name: &'static str) -> (WorkerGuard, WorkerGuard) {
    let proj_dir = ProjectDirs::from(
        "io",  // qualifier
        "i01", // organization
        name,  // application
    )
    .expect("Failed to set project directory");

    let file_appender = tracing_appender::rolling::daily(proj_dir.cache_dir(), "search.log");
    let (file_log, file_guard) = tracing_appender::non_blocking(file_appender);
    let (stdout_log, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(file_log)
        .with_filter(filter::filter_fn(move |metadata| metadata.target() == name));

    let env_filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(format!("{name}=warn")))
        .unwrap();
    let stdout_layer = fmt::layer()
        .with_writer(stdout_log)
        .with_filter(env_filter_layer);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .init();

    (file_guard, stdout_guard)
}
