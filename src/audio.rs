use std::{
    io::{BufReader, Cursor},
    str::FromStr,
};

use anyhow::{bail, Result};
use rodio::{Decoder, OutputStream, Sink};
use tokio::task;
use tracing::info;

use crate::USER_AGENT;

const AUDIO_API: &str = "https://dict.youdao.com/dictvoice?audio=";

#[derive(Debug, Default, Clone, Copy)]
pub enum Pron {
    #[default]
    UK,
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
    pub fn api_url(&self, word: &str) -> String {
        match self {
            Self::UK => format!("{AUDIO_API}{word}&type=1"),
            Self::US => format!("{AUDIO_API}{word}&type=2"),
            Self::JAP => format!("{AUDIO_API}{word}&le=jap"),
        }
    }

    pub async fn say(&self, word: &str) -> Result<()> {
        let api_url = self.api_url(word);
        info!(api_url);
        let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
        let response = client.get(api_url).send().await?.bytes().await?;
        info!("got response");

        task::spawn_blocking(move || {
            play(BufReader::new(Cursor::new(response.to_vec()))).expect("play error");
        })
        .await?;

        info!("after play");

        Ok(())
    }
}

fn play<T>(buf: BufReader<T>) -> Result<()>
where
    T: std::io::Read + std::io::Seek + std::marker::Send + std::marker::Sync + 'static,
{
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle).unwrap();
    info!("get output stream");
    let source = Decoder::new(buf).unwrap();
    info!("get decoded source");
    // Play the sound directly on the device
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}
