use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Usage: {0} <command>")]
    ErrorParsingArgs(String),
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("not a git repository (or any parent up to mount point /)")]
    NotARepository,

    #[error("not a git repository: {0}")]
    NotARepositoryAt(PathBuf),

    #[error("repository already exists: {0}")]
    AlreadyExists(PathBuf),

    #[error("failed to initialize repository")]
    InitFailed,

    // =========================================================================
    // IO ERRORS
    // =========================================================================
    #[error("failed to create directory: {0}")]
    CreateDir(PathBuf, #[source] io::Error),

    #[error("failed to write file: {0}")]
    WriteFile(PathBuf, #[source] io::Error),

    #[error("failed to read file: {0}")]
    ReadFile(PathBuf, #[source] io::Error),

    #[error(transparent)]
    Io(#[from] io::Error),
}
