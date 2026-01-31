use crate::utils::error::RepositoryError;
use std::fs;
use std::path::Path;

pub struct Repository {}

impl Repository {
    pub fn init() -> Result<(), RepositoryError> {
        let git_dir = Path::new(".git");

        if git_dir.exists() {
            return Err(RepositoryError::AlreadyExists(git_dir.to_path_buf()));
        }

        fs::create_dir(".git").map_err(|e| RepositoryError::CreateDir(".git".into(), e))?;

        fs::create_dir(".git/objects")
            .map_err(|e| RepositoryError::CreateDir(".git/objects".into(), e))?;

        fs::create_dir(".git/refs")
            .map_err(|e| RepositoryError::CreateDir(".git/refs".into(), e))?;

        fs::write(".git/HEAD", "ref: refs/heads/main\n")
            .map_err(|e| RepositoryError::WriteFile(".git/HEAD".into(), e))?;

        Ok(())
    }
}
