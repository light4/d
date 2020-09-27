#[macro_use]
extern crate log;

use anyhow::Result;
use async_process::Command;
use async_std;
use clap::{App, Arg};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use regex::Regex;
use surf;

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
    println!("How to say {}?", word);

    Command::new("espeak-ng").arg(word).output().await?;
    Ok(())
}

fn init_log() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(local)} - {M} - {m}{n}",
        )))
        .build("/tmp/d.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

#[async_std::main]
async fn main() -> Result<()> {
    init_log();
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
