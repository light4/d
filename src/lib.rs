use std::{
    io::{BufReader, Cursor},
    str::FromStr,
};

use anyhow::{bail, Result};
use regex::Regex;
use reqwest::Url;
use tokio::task;
use tracing::info;

pub mod audio;
pub mod telemetry;

const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0";
const SEARCH_API: &str = "https://m.youdao.com/dict";
const AUDIO_API: &str = "https://dict.youdao.com/dictvoice?audio=";

fn build_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    Ok(client)
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Pron {
    UK,
    #[default]
    US,
    JAP,
}

impl FromStr for Pron {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "uk" => Ok(Self::UK),
            "us" => Ok(Self::US),
            "jap" => Ok(Self::JAP),
            _ => bail!("parse pron option error"),
        }
    }
}

impl Pron {
    pub fn audio_url(&self, word: &str) -> String {
        match self {
            Self::UK => format!("{AUDIO_API}{word}&type=1"),
            Self::US => format!("{AUDIO_API}{word}&type=2"),
            Self::JAP => format!("{AUDIO_API}{word}&le=jap"),
        }
    }

    pub fn search_url(&self, word: &str) -> Url {
        match self {
            Self::UK => Url::parse_with_params(SEARCH_API, &[("q", word), ("le", "eng")]).unwrap(),
            Self::US => Url::parse_with_params(SEARCH_API, &[("q", word), ("le", "eng")]).unwrap(),
            Self::JAP => Url::parse_with_params(SEARCH_API, &[("q", word), ("le", "jap")]).unwrap(),
        }
    }

    pub async fn say(&self, word: &str) -> Result<()> {
        let api_url = self.audio_url(word);
        info!(api_url);
        let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
        let response = client.get(api_url).send().await?.bytes().await?;
        info!("got response");

        task::spawn_blocking(move || {
            audio::play(BufReader::new(Cursor::new(response.to_vec()))).expect("play error");
        })
        .await?;

        info!("after play");

        Ok(())
    }

    pub async fn search(&self, word: &str) -> Result<()> {
        let url = self.search_url(word);
        let client = build_client()?;
        let body = client.get(url).send().await?.text().await?;

        let re = Regex::new(r"(?s)/h2>.*?<ul.*?/ul>")?;
        if let Some(mat) = re.find(&body) {
            let re = Regex::new(r"(?s:<li>)(.*?)(?:</li>)")?;
            for cap in re.captures_iter(mat.as_str()) {
                println!("{}", &cap[1]);
            }
        } else {
            println!("Aha! Couldn't find {word}.");
        }

        Ok(())
    }
}
