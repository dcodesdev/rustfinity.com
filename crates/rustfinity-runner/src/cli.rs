use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about = "CLI for rust code runner", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Run and test the code based on the challenge and code provided")]
    Run {
        #[clap(long)]
        /// Code base64 encoded
        code: String,

        #[clap(long)]
        /// Challenge slug
        challenge: Option<String>,

        #[clap(long, short)]
        /// number of tests to take the minimum time of
        n_tests: Option<usize>,
    },
}
