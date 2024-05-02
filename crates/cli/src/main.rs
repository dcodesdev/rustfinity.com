mod challenge;
mod cli;
mod download;

use clap::Parser;
use cli::{run, Cli};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    run(cli).await;
}
