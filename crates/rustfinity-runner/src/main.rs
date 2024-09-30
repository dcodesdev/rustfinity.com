use clap::Parser;
use command::{run_code, RunCodeParams};
use dotenvy::dotenv;

mod cli;
mod command;
mod regex;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            code: code_base64,
            challenge,
            n_tests,
        } => {
            let params = RunCodeParams::new(code_base64, challenge, n_tests);

            match run_code(&params).await {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("{}", e),
            };
        }
    }
}
