//! Errors, son!

use std::fmt;
use std::io;
use std::error;
use std::result;
use std::string::FromUtf8Error;

use serde::ser;

/// The errors that can arise.
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorCode {
    /// Catchall syntax for error messages
    Custom(String),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::Custom(ref msg) => write!(f, "{}", msg),
        }
    }
}

/// Represents all possible errors that can occur when serializing or deserializing a value into
/// bencode.
#[derive(Debug)]
pub enum Error {
    Syntax(ErrorCode),

    Io(io::Error),

    Utf8(FromUtf8Error)
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Syntax(..) => "syntax error",
            Error::Io(..) => "io error",
            Error::Utf8(..) => "utf-8 error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref code) => {
                write!(fmt, "{}", code)
            },
            Error::Io(ref err) => {
                write!(fmt, "{}", err)
            },
            Error::Utf8(ref err) => {
                write!(fmt, "{}", err)
            }
        }
    }
}

impl ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Syntax(ErrorCode::Custom(msg.into()))
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::Utf8(error)
    }
}

/// Helper alias for `Result` objects that return a JSON `Error`.
pub type Result<T> = result::Result<T, Error>;
