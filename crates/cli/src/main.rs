mod challenge;
mod cli;
mod download;

use clap::Parser;
use cli::{run, Cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli).await
}
