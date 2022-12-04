use anyhow::Result;
use clap::Parser;
use d::telemetry::setup_log;
use tracing::{error, info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    /// word to query
    word: String,
    /// word to query
    pron: Option<d::audio::Pron>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = setup_log("d", "info");

    let opt = Options::parse();
    let word = opt.word;
    info!("{}", &word);
    let word_clone = word.clone();
    opt.pron
        .unwrap_or_default()
        .say(&word_clone)
        .await
        .unwrap_or(error!("cannot say word: {}", &word_clone));
    d::search(&word).await?;

    Ok(())
}
