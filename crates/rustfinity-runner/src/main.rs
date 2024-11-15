use clap::Parser;
use command::{run_code, RunCodeParams};
use dotenvy::dotenv;

mod cli;
mod command;
mod playground;
mod regex;
mod utils;

use cli::{Cli, Commands};
use playground::{run_code_in_playground, PlaygroundParams};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Test {
            code: code_base64,
            tests: tests_base64,
            cargo_toml: cargo_toml_base64,
            n_tests,
        } => {
            let params =
                RunCodeParams::new(code_base64, tests_base64, cargo_toml_base64, false, n_tests);

            run_code(&params).await
        }

        Commands::Playground { code: code_base64 } => {
            let params = PlaygroundParams::new(code_base64);

            run_code_in_playground(&params).await
        }
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("{}", e),
    }
}
