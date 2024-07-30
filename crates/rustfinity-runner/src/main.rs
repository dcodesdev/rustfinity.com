use clap::Parser;
use command::run_code;
use dotenvy::dotenv;

mod cli;
mod command;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            code: code_base64,
            challenge,
        } => {
            match run_code(&code_base64, &challenge).await {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("{}", e),
            };
        }
    }
}
