use clap::Parser;
use codecrafters_git::Args;

fn main() {
    if let Err(e) = Args::parse().exec() {
        eprintln!("{e}");
    }
}
