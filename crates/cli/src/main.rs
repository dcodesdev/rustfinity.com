mod cargo_toml;
mod challenge;
mod cli;
mod constants;
mod crates_io;
mod download;

use clap::Parser;
use cli::{run, Cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let latest_version = crates_io::get_latest_version().await?;
    let current_version = env!("CARGO_PKG_VERSION");

    if latest_version != current_version {
        println!(
            "A new version of rustfinity ({}) is available, please run the following command and try again:\n\n$ cargo install rustfinity",
            latest_version
        );
        return Ok(());
    }

    let cli = Cli::parse();
    run(cli).await
}
