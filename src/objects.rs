use std::str::FromStr;
use thiserror::Error;

pub struct Object {
    pub kind: ObjectKind,
    pub bytes: Vec<u8>,
}

pub enum ObjectKind {
    Blob,
}

impl FromStr for ObjectKind {
    type Err = ObjectParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blob" => Ok(Self::Blob),
            _ => Err(ObjectParseError::InvalidObjectKind(s.into())),
        }
    }
}

#[derive(Error, Debug)]
pub enum ObjectParseError {
    #[error("invalid object kind : {0}")]
    InvalidObjectKind(String),
}
