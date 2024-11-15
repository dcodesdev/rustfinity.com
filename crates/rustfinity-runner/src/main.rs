use clap::Parser;
use cli::{Cli, Commands};
use commands::{
    playground::{run_code_in_playground, PlaygroundParams},
    run_tests::{run_tests, RunTestsParams},
};
use dotenvy::dotenv;

mod cli;
mod commands;
mod regex;
mod utils;

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
            let params = RunTestsParams::new(code_base64, tests_base64, cargo_toml_base64, n_tests);

            run_tests(&params).await
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
