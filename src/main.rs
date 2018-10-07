#[macro_use]
extern crate log;
extern crate log4rs;
extern crate reqwest;
extern crate failure;
extern crate clap;
extern crate regex;

use failure::Error;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use std::io::Read;
use clap::{Arg, App};
use regex::Regex;
use reqwest::header::USER_AGENT;
use std::thread;

fn search(word: &str) -> Result<(), Error> {
    let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0";
    let url = format!("http://m.youdao.com/dict?le=eng&q={}", word);
    let client = reqwest::Client::new();
    let mut res = client.get(&url)
        .header(USER_AGENT, user_agent)
        .send()?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

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

#[cfg(target_os="macos")]
fn say(word: &str) {
    use std::process::Command;
    let err = format!("Failed to say {:?}", word);
    Command::new("say")
            .arg(word)
            .output()
            .expect(&err);
}

#[cfg(not(target_os="macos"))]
fn say(word: &str) {
    println!("How to say {}?", word);
}

fn init_log() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S %Z)(local)} - {M} - {m}{n}")))
        .build("/tmp/d.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();
}

fn main() {
    init_log();
    let matches = App::new("A Tiny Dictionary For Myself")
        .version("0.1.0")
        .author("Light Ning <lightning1141@gmail.com>")
        .about("A Tiny Dictionary For Myself")
        .arg(Arg::with_name("WORD")
                 .help("Word to search")
                 .required(true)
                 .index(1))
        .get_matches();
    let word = matches.value_of("WORD")
                      .unwrap_or("shush")
                      .to_owned();
    info!("{}", word);
    let mut children = vec![];
    let word_s = word.clone();
    children.push(thread::spawn(move || {
        say(&word_s);
    }));
    children.push(thread::spawn(move || {
        match search(&word) {
            Ok(_) => {},
            Err(_) => println!("Aha!"),
        };
    }));
    for child in children {
        let _ = child.join();
    }
}
