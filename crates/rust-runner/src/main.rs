use clap::Parser;
use command::run_code;

mod cli;
mod command;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            code: code_base64,
            challenge,
        } => {
            dbg!(code_base64, challenge);
        }
    }

    let code_base64 = "cHViIGZuIGhlbGxvX3dvcmxkKCkgewogICAgcHJpbnRsbiEoImhlbGxvLCB3b3JsZCIpCn0K";
    let challenge = "printing-hello-world";

    match run_code(code_base64, challenge).await {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("{}", e),
    };
}
