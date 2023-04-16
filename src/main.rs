use anyhow::Result;
use clap::Parser;
use d::{get_pron_from, telemetry::setup_log, Pron};
use tracing::{error, info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    /// word to query
    word: String,
    /// word to query
    pron: Option<Pron>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let (_file_guard, _stdout_guard) = setup_log(env!("CARGO_PKG_NAME"));

    let opt = Options::parse();
    let word = opt.word;
    info!("searching: {}", &word);
    let word_clone = word.clone();
    dbg!(get_pron_from(&word));
    let pron = opt.pron.unwrap_or_else(|| get_pron_from(&word));
    pron.search(&word).await?;
    if let Err(e) = pron.say(&word_clone).await {
        error!("cannot say word {}: {}", &word_clone, e);
    }

    Ok(())
}
