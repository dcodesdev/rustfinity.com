mod challenge;
mod cli;
mod download;

use clap::Parser;
use cli::{run, Cli};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match run(cli).await {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
