use std::path::Path;

use super::error::CommandError;
use crate::repo::Repository;

pub fn init(root: &Path) -> Result<(), CommandError> {
    let repo = Repository::new(root)?;
    println!(
        "Initialized empty Git repository in {}",
        repo.directory.display()
    );
    Ok(())
}
