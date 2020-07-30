use std::error::Error as StdError;
use std::{fmt, io};

#[derive(Debug)]
pub struct Error(Box<ErrorKind>, Option<String>);
impl Error {
    pub fn new(kind: ErrorKind, message: Option<String>) -> Error {
        Error(Box::new(kind), message)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidInput(String),
    CSV(csv::Error),
    Io(io::Error, Option<String>),
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::new(ErrorKind::InvalidInput(err), None)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Error {
        Error::new(ErrorKind::CSV(err), None)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err, None), None)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self.0 {
            ErrorKind::Io(ref err, None) => Some(err),
            ErrorKind::CSV(ref err) => Some(err),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err, ref message) => match message {
                Some(message) => write!(f, "{} {}", message, err),
                _ => write!(f, "{}", err),
            },
            ErrorKind::InvalidInput(ref s) => write!(f, "{}", s),
            ErrorKind::CSV(ref err) => match *err.kind() {
                csv::ErrorKind::Io(ref err) => write!(f, "{}", err),
                _ => write!(f, "{}", err),
            },
        }
    }
}
