use crate::download::get_challenge;
use clap::{Parser, Subcommand};

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Get { command: get } => match get {
            Get::Challenge { challenge } => get_challenge(&challenge).await,
        },
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,

    /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        #[command(subcommand)]
        command: Get,
    },
}

#[derive(Subcommand)]
enum Get {
    Challenge { challenge: String },
}
