use clap::Parser;
use shorten::{parser::Cli, Shorten};
mod parser;
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.short {
        Some(link) => {
            let short_link = Shorten::new(&link).short_link().await;
            println!("{}", short_link);
        }
        None => {
            if let Some(link) = &cli.long {
                let long_link = Shorten::new(link).long_link().await;
                println!("{}", long_link);
            }
        }
    }
}
