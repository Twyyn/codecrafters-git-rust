use super::error::CommandError;
use crate::repo::Repository;
use std::io::{self, Write};

pub fn cat_file(repo: &Repository, hash: &str) -> Result<(), CommandError> {
    let object = repo.read_object(hash)?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&object.bytes)?;
    stdout.flush()?;

    Ok(())
}
