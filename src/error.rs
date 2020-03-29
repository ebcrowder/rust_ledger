use std::io::{Error, ErrorKind};

/// returns an error if a command is not provided
pub fn error() -> Result<(), std::io::Error> {
    Err(Error::new(
        ErrorKind::InvalidInput,
        "invalid input: please provide a command",
    ))
}
