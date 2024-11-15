use clap::Parser;
use command::{run_code, RunCodeParams};
use dotenvy::dotenv;

mod cli;
mod command;
mod playground;
mod regex;
mod utils;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Test {
            code: code_base64,
            tests: tests_base64,
            cargo_toml: cargo_toml_base64,
            n_tests,
        } => {
            let params =
                RunCodeParams::new(code_base64, tests_base64, cargo_toml_base64, false, n_tests);

            match run_code(&params).await {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("{}", e),
            };
        }
        Commands::Playground { code: code_base64 } => {
            let params =
                RunCodeParams::new(code_base64, "".to_string(), "".to_string(), true, None);

            match run_code(&params).await {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("{}", e),
            };
        }
    }
}
