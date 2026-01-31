use crate::core::repository::Repository;
use crate::utils::error::RepositoryError;

#[derive(Debug)]
pub struct Init;

pub fn execute() -> Result<(), RepositoryError> {
    Repository::init()?;

    println!("Initialized git directory");
    Ok(())
}
