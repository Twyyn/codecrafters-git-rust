use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Object(#[from] crate::objects::ObjectError),

    #[error(transparent)]
    Repo(#[from] crate::repo::RepoError),

    #[error("object file does not exist: {0:?}")]
    ObjectFile(String),
}
