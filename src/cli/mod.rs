pub mod commands;
use crate::utils::error::ParseError;
use std::env;

pub struct Args {
    pub command: String,
}

impl Args {
    pub fn parse() -> Result<Self, ParseError> {
        let mut args = env::args();

        let program = args.next().unwrap();
        let command = match args.next() {
            Some(command) => command,
            None => {
                return Err(ParseError::ErrorParsingArgs(program));
            }
        };

        Ok(Args { command })
    }
}
