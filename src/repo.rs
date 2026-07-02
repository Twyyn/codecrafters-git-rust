use crate::objects::{Object, ObjectKind};
use flate2::read::ZlibDecoder;
use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::{fs, fs::File};
use thiserror::Error;

const GIT_DIRECTORY: &str = ".git";
const OBJECTS_DIRECTORY: &str = "objects";
const REFS_DIRECTORY: &str = "refs";
const HEAD_FILE: &str = "HEAD";

pub struct Repository {
    pub directory: PathBuf,
}

impl Repository {
    pub fn new(root: &Path) -> Result<Self, RepoError> {
        let directory = root.join(GIT_DIRECTORY);

        fs::create_dir_all(directory.join(OBJECTS_DIRECTORY))?;
        fs::create_dir_all(directory.join(REFS_DIRECTORY))?;
        fs::write(directory.join(HEAD_FILE), "ref: refs/heads/main\n")?;

        Ok(Self { directory })
    }

    pub fn open(root: &Path) -> Result<Self, RepoError> {
        let directory = root.join(GIT_DIRECTORY);

        if !directory.is_dir() {
            return Err(RepoError::NotARepository(directory.display().to_string()));
        }

        Ok(Self { directory })
    }

    pub fn read_object(&self, hash: &str) -> Result<Object, RepoError> {
        let path = self.discover_object(hash);

        let file = File::open(&path).map_err(RepoError::Io)?;
        let mut reader = BufReader::new(ZlibDecoder::new(file));

        Ok(Object::try_from(&mut reader)?)
    }

    fn discover_object(&self, hash: &str) -> PathBuf {
        let (object_directory, object_filename) = hash.split_at(2);
        self.directory
            .join(OBJECTS_DIRECTORY)
            .join(object_directory)
            .join(object_filename)
    }
}

#[derive(Error, Debug)]
pub enum RepoError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Object(#[from] crate::objects::ObjectError),

    #[error("object file does not exist: {0:?}")]
    ObjectFile(String),

    #[error("not a git repository: {0}")]
    NotARepository(String),
}
