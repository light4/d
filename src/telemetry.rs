use directories::ProjectDirs;

use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

pub fn setup_log<S: AsRef<str>>(name: S, env_filter: S) -> WorkerGuard {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let proj_dir = ProjectDirs::from(
        "io",          /*qualifier*/
        "i01",         /*organization*/
        name.as_ref(), /*application*/
    )
    .expect("Failed to set project directory");

    let file_appender = tracing_appender::rolling::daily(proj_dir.cache_dir(), "search.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .finish();

    set_global_default(subscriber).expect("Failed to set subscriber");

    guard
}
