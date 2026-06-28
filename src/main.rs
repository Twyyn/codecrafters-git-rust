use clap::Parser;
use codecrafters_git::Args;

fn main() {
    if let Err(e) = Args::parse().run() {
        eprintln!("{e}");
    }
}
