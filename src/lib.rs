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
    fn new(url: &str) -> Self {
        Self {
            url: format!("https://api.shrtco.de/v2/shorten?url={}", url),
        }
    }

    async fn shorten_link(&self) -> Result<Links, Box<dyn std::error::Error>> {
        let response = reqwest::get(&self.url).await?.json().await?;
        Ok(response)
    }
}
#[cfg(test)]
mod tests {
    use crate::Shorten;

    #[tokio::test]
    async fn it_works() {
        let shorten = Shorten::new("google.com");
        let links = shorten.shorten_link().await;
        println!("{}", links.unwrap().result.full_short_link);
    }
}
