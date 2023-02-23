use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "shorten")]
#[command(author = "MrLectus. <mrlectus@tutanota.com>")]
#[command(version = "1.0")]
#[command(about = "shortens url links", long_about = None)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub short: Option<String>,
    #[arg(long)]
    pub long: Option<String>,
}
