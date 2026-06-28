use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Repository {
    directory: PathBuf,
}

impl Repository {
    pub fn new(root: &Path) -> Result<Self> {
        let directory = root.join(".git");
        fs::create_dir_all(directory.join("objects"))?;
        fs::create_dir_all(directory.join("refs"))?;
        fs::write(directory.join("HEAD"), "ref: refs/heads/main\n")?;
        Ok(Self { directory })
    }
}
