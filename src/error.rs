use std::fmt;
use std::io;
use std::result;

extern crate csv;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Csv(csv::Error),
    InvalidArg(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref err) => write!(f, "{}", err),
            Error::Csv(ref err) => write!(f, "{}", err),
            Error::InvalidArg(ref s) => write!(f, "{}", s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Error {
        Error::Csv(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::InvalidArg(err)
    }
}
