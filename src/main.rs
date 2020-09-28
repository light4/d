use d::telemetry::setup_log;

use anyhow::Result;
use async_process::Command;
use async_std;
use clap::{App, Arg};
use regex::Regex;
use surf;
use tracing::{info, warn};

async fn search(word: &str) -> Result<()> {
    let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0";
    let url = format!("http://m.youdao.com/dict?le=eng&q={}", word);
    let body = surf::get(&url)
        .set_header("User-Agent", user_agent)
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
async fn say(word: &str) -> Result<()> {
    let err = format!("Failed to say {:?}", word);
    Command::new("say").arg(word).output().await?;

    Ok(())
}

#[cfg(not(target_os = "macos"))]
async fn say(word: &str) -> Result<()> {
    warn!("How to say {}?", word);

    Command::new("espeak-ng").arg(word).output().await?;
    Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
    let _guard = setup_log("d", "info");

    let matches = App::new("A Tiny Dictionary For Myself")
        .version("0.1.0")
        .author("Light Ning <lightning1141@gmail.com>")
        .about("A Tiny Dictionary For Myself")
        .arg(
            Arg::with_name("WORD")
                .help("Word to search")
                .required(true)
                .index(1),
        )
        .get_matches();
    let word = matches.value_of("WORD").unwrap_or("shush").to_owned();
    info!("{}", &word);
    say(&word).await?;
    search(&word).await?;

    Ok(())
}
