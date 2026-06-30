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

        let file = File::open(&path).map_err(|e| match e.kind() {
            ErrorKind::NotFound => RepoError::InvalidObjectFile(path.display().to_string()),
            _ => RepoError::Io(e),
        })?;

        let decoder = ZlibDecoder::new(file);
        let mut reader = BufReader::new(decoder);

        let mut bytes = Vec::new();
        reader.read_until(0, &mut bytes)?;

        if bytes.pop() != Some(0) {
            return Err(RepoError::InvalidObjectHeaderFormat);
        }

        let header = std::str::from_utf8(&bytes)?;
        let Some((kind, size)) = header.split_once(' ') else {
            return Err(RepoError::InvalidObjectHeader(header.into()));
        };

        let kind = kind
            .parse::<ObjectKind>()
            .map_err(|_| RepoError::InvalidObjectKind(kind.into()))?;

        let size = size
            .parse::<usize>()
            .map_err(|_| RepoError::InvalidObjectSize(size.into()))?;

        let mut bytes = vec![0u8; size];
        reader.read_exact(&mut bytes)?;

        Ok(Object { kind, bytes })
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
    Utf8(#[from] std::str::Utf8Error),

    #[error("object file does not exist: {0:?}")]
    InvalidObjectFile(String),

    #[error("object header is not NUL-terminated")]
    InvalidObjectHeaderFormat,

    #[error("malformed object header: {0:?}")]
    InvalidObjectHeader(String),

    #[error("invalid object size: {0:?}")]
    InvalidObjectSize(String),

    #[error("invalid object kind: {0:?}")]
    InvalidObjectKind(String),

    #[error("not a git repository: {0}")]
    NotARepository(String),
}
