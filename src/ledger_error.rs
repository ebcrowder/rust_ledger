use std::fmt;
use std::io;

extern crate csv;

#[derive(Debug)]
pub enum LedgerError {
    InputError(io::Error),
    CSVError(csv::Error),
    Other(String),
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LedgerError::InputError(ref err) => {
                write!(f, "Input error. Argument is not correct: {}", err)
            }
            LedgerError::CSVError(ref err) => write!(f, "CSV error: {}", err),
            LedgerError::Other(ref s) => write!(f, "other error: {}", s),
        }
    }
}

impl From<io::Error> for LedgerError {
    fn from(err: io::Error) -> LedgerError {
        LedgerError::InputError(err)
    }
}

impl From<csv::Error> for LedgerError {
    fn from(err: csv::Error) -> LedgerError {
        LedgerError::CSVError(err)
    }
}

impl From<String> for LedgerError {
    fn from(err: String) -> LedgerError {
        LedgerError::Other(err)
    }
}
