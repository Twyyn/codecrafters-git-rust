use codecrafters_git::{cli::Args, fs::init_git_dir};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command.as_str() {
        "init" => {
            init_git_dir()?;
            println!("Initialized git directory")
        }

        _ => println!("Unknown command: {}", args.command),
    }

    Ok(())
}
