use std::{process::Command, thread};

use anyhow::Result;
use clap::Parser;
use d::telemetry::setup_log;
use regex::Regex;
use tracing::{error, info, warn};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// word to query
    word: String,
}

async fn search(word: &str) -> Result<()> {
    let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0";
    let url = format!("http://m.youdao.com/dict?le=eng&q={}", word);
    let body = surf::get(&url)
        .header("User-Agent", user_agent)
        .recv_string()
        .await
        .unwrap();

    let re = Regex::new(r"(?s)/h2>.*?<ul.*?/ul>").unwrap();
    if let Some(mat) = re.find(&body) {
        let re = Regex::new(r"(?s:<li>)(.*?)(?:</li>)").unwrap();
        for cap in re.captures_iter(mat.as_str()) {
            println!("{}", &cap[1]);
        }
    } else {
        println!("Aha! Couldn't find {}.", word);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn say(word: &str) -> Result<()> {
    let err = format!("Failed to say {:?}", word);
    Command::new("say").arg(word).output()?;

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn say(word: &str) -> Result<()> {
    warn!("How to say {}?", word);

    Command::new("espeak-ng").arg(word).output()?;
    Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
    let _guard = setup_log("d", "info");

    let cli = Cli::parse();
    let word = cli.word;
    info!("{}", &word);
    let word_clone = word.clone();
    let handler = thread::spawn(move || {
        say(&word_clone).unwrap_or(error!("cannot say word: {}", &word_clone));
    });
    search(&word).await?;
    handler.join().unwrap();

    Ok(())
}
