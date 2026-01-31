use codecrafters_git::cli::Args;
use codecrafters_git::cli::commands::init;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse()?;

    match args.command.as_str() {
        "init" => {
            init::execute()?;
        }

        _ => println!("Unknown command: {}", args.command),
    }

    Ok(())
}
