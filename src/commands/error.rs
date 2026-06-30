use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ObjectParseError(#[from] crate::objects::ObjectParseError),

    #[error(transparent)]
    RepoError(#[from] crate::repo::RepoError),

    #[error("object file does not exist: {0:?}")]
    InvalidObjectFile(String),
}
