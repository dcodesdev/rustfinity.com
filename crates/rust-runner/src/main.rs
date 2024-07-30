use clap::Parser;
use command::run_code;

mod cli;
mod command;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { code: code_base64 } => {
            dbg!(code_base64);
        }
    }

    let code_base64 = "c3JjX2luY2x1ZGUoKSB7CiAgICBwcmludCgiSGVsbG8gV29ybGQiKQogIH0=";
    let challenge = "hello_world";

    match run_code(code_base64, challenge).await {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("{}", e),
    };
}
