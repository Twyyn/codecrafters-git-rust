use std::{env, process};

pub struct Args {
    pub command: String,
}

impl Args {
    pub fn parse() -> Self {
        let mut args = env::args();

        let program = args.next().unwrap();
        let command = match args.next() {
            Some(command) => command,
            None => {
                eprintln!("Usage: {} <command>", program);
                process::exit(64);
            }
        };

        Args { command }
    }
}
