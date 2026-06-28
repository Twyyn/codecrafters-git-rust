use crate::repository::Repository;
use anyhow::Result;

pub fn run() -> Result<()> {
    Repository::new(&std::env::current_dir()?)?;
    println!("Initialized git directory");
    Ok(())
}
