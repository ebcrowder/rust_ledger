use std::error;
use std::fmt;
use std::io::Error;

extern crate csv;

#[derive(Debug)]
pub enum LedgerError {
    InputError(Error),
    CSVError(csv::Error),
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LedgerError::InputError(ref err) => {
                write!(f, "Input error. Argument is not correct: {}", err)
            }
            LedgerError::CSVError(ref err) => write!(f, "CSV error: {}", err),
        }
    }
}

impl error::Error for LedgerError {
    fn description(&self) -> &str {
        match *self {
            LedgerError::InputError(ref err) => err.description(),
            LedgerError::CSVError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LedgerError::InputError(ref err) => Some(err),
            LedgerError::CSVError(ref err) => Some(err),
        }
    }
}

impl From<Error> for LedgerError {
    fn from(err: Error) -> LedgerError {
        LedgerError::InputError(err)
    }
}

impl From<csv::Error> for LedgerError {
    fn from(err: csv::Error) -> LedgerError {
        LedgerError::CSVError(err)
    }
}
