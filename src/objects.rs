use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub enum Kind {
    Blob,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blob => "blob",
        }
    }
}

impl FromStr for Kind {
    type Err = ObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blob" => Ok(Self::Blob),
            unknown => Err(ObjectError::ObjectKind(unknown.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct Object {
    pub kind: Kind,
    pub content: Vec<u8>,
}

impl Object {
    pub fn decode<R: BufRead>(mut reader: R) -> Result<Self, ObjectError> {
        let mut header = Vec::with_capacity(32);
        reader.read_until(0, &mut header)?;

        if header.pop() != Some(0) {
            return Err(ObjectError::ObjectHeaderFormat);
        }

        let header = std::str::from_utf8(&header)?;
        let Some((kind, size)) = header.split_once(' ') else {
            return Err(ObjectError::ObjectHeader(header.into()));
        };

        let kind = kind
            .parse()
            .map_err(|_| ObjectError::ObjectKind(kind.to_owned()))?;

        let size = size
            .parse()
            .map_err(|_| ObjectError::ObjectSize(size.to_owned()))?;

        let mut content = vec![0; size];
        reader.read_exact(&mut content)?;

        Ok(Self { kind, content })
    }
}

#[derive(Error, Debug)]
pub enum ObjectError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Size(#[from] std::str::Utf8Error),

    #[error("object header is not NUL-terminated")]
    ObjectHeaderFormat,

    #[error("malformed object header: {0:?}")]
    ObjectHeader(String),

    #[error("invalid object size: {0:?}")]
    ObjectSize(String),

    #[error("invalid object kind: {0:?}")]
    ObjectKind(String),

    #[error("not a git repository: {0}")]
    NotARepository(String),
}
