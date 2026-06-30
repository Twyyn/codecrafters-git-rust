mod cat_file;
mod error;
mod init;

use std::path::Path;

use clap::{Parser, Subcommand};
use error::CommandError;

use crate::repo::Repository;

#[derive(Debug, Parser)]
#[command(name = "git")]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    CatFile {
        #[arg(short = 'p', required = true)]
        pretty: bool,
        hash: String,
    },
}

impl Args {
    pub fn exec(self) -> Result<(), CommandError> {
        match self.command {
            Commands::Init => init::init(Path::new(".")),
            Commands::CatFile { hash, .. } => {
                let repo = Repository::open(Path::new("."))?;
                cat_file::cat_file(&repo, &hash)
            }
        }
    }
}
