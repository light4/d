use anyhow::Result;
use regex::Regex;

pub mod audio;
pub mod telemetry;

const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0";

pub async fn search(word: &str) -> Result<()> {
    let url = format!("http://m.youdao.com/dict?le=eng&q={word}");
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let body = client.get(&url).send().await?.text().await?;

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
