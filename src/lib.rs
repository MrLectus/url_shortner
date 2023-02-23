pub mod parser;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    code: String,
    short_link: String,
    full_short_link: String,
    short_link2: String,
    full_short_link2: String,
    short_link3: String,
    full_short_link3: String,
    share_link: String,
    full_share_link: String,
    original_link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    ok: bool,
    result: Results,
}

#[derive(Debug)]
pub struct Shorten {
    url: String,
}

impl Shorten {
    pub fn new(url: &str) -> Self {
        Self {
            url: format!("https://api.shrtco.de/v2/shorten?url={}", url),
        }
    }

    pub async fn shortend_link(&self) -> Result<Links, Box<dyn std::error::Error>> {
        let response = reqwest::get(&self.url).await?.json().await?;
        Ok(response)
    }

    pub async fn short_link(&self) -> String {
        let shorten = Shorten::new(&self.url);
        let links = shorten.shortend_link().await;
        match links {
            Ok(links) => links.result.short_link,
            Err(_) => "".to_string(),
        }
    }
    pub async fn long_link(&self) -> String {
        let shorten = Shorten::new(&self.url);
        let links = shorten.shortend_link().await;
        match links {
            Ok(links) => links.result.full_short_link,
            Err(_) => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Shorten;

    #[tokio::test]
    async fn it_works() {
        println!(
            "{}",
            Shorten::new("https://www.google.com").short_link().await
        );
    }
}
