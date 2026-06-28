mod init;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "git")]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
}

impl Args {
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Init => init::run(),
        }
    }
}
