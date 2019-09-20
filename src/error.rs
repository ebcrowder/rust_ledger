// returns an error if a command is not provided

use std::io::{Error, ErrorKind};

pub fn error() -> Result<(), std::io::Error> {
    Err(Error::new(
        ErrorKind::InvalidInput,
        "invalid input: please provide a command",
    ))
}
