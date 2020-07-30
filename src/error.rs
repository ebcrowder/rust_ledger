use std::fmt;
use std::io;

extern crate csv;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    CSVError(csv::Error),
    InvalidArg(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IOError(ref err) => write!(f, "{}", err),
            Error::CSVError(ref err) => write!(f, "{}", err),
            Error::InvalidArg(ref s) => write!(f, "{}", s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Error {
        Error::CSVError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::InvalidArg(err)
    }
}
