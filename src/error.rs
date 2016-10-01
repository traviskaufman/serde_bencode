//! Errors, son!

use std::fmt;
use std::io;
use std::error;
use std::result;
use std::string::FromUtf8Error;

use serde::de;
use serde::ser;

/// The errors that can arise.
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorCode {
    /// Default error code for when the parser encounters a malformed message
    UnexpectedToken(String),
    /// Used when the deserializer hits the end of input when it's not expecting it
    UnexpectedEOF,
    /// Used when there are remaining characters after deserializing from an iterator
    UnexpectedTrailingChars,
    /// Used when the serializer cannot serialize the given type
    UnsupportedType(de::Type),
    /// Used when trying to serialize a number that cannot be bencoded
    NumberOutOfRange(u64),
    /// Catchall syntax for error messages
    Custom(String),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::UnexpectedToken(ref tok) => write!(f, "Unexpected token {}", tok),
            ErrorCode::UnexpectedEOF => write!(f, "Unexpected end of input"),
            ErrorCode::UnexpectedTrailingChars => write!(f, "Unexpected trailing characters"),
            ErrorCode::UnsupportedType(ref t) => write!(f, "Cannot serialize type {}", t),
            ErrorCode::NumberOutOfRange(ref n) => write!(f, "Number {} out of range", n),
            ErrorCode::Custom(ref msg) => write!(f, "{}", msg),
        }
    }
}

/// Represents all possible errors that can occur when serializing or deserializing a value into
/// bencode.
#[derive(Debug)]
pub enum Error {
    Syntax(ErrorCode, usize),

    Io(io::Error),

    Utf8(FromUtf8Error),

    /// Used by the serializer when encountering types it cannot serialize
    Ser(ErrorCode),

    Value(de::value::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Syntax(..) => "Syntax error",
            Error::Io(..) => "io error",
            Error::Utf8(..) => "utf-8 error",
            Error::Ser(..) => "Serialization error",
            Error::Value(..) => "Value error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::Value(ref err) => Some(err),
            _ => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref code, pos) => write!(fmt, "At position {}: {}", pos, code),
            Error::Io(ref err) => write!(fmt, "{}", err),
            Error::Utf8(ref err) => write!(fmt, "{}", err),
            Error::Value(ref err) => write!(fmt, "{}", err),
            Error::Ser(ref code) => write!(fmt, "{}", code),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Syntax(ErrorCode::Custom(msg.into()), 0)
    }
}

impl de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Syntax(ErrorCode::Custom(msg.into()), 0)
    }

    fn end_of_stream() -> Error {
        Error::Syntax(ErrorCode::UnexpectedEOF, 0)
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

impl From<de::value::Error> for Error {
    fn from(error: de::value::Error) -> Self {
        Error::Value(error)
    }
}

/// Helper alias for `Result` objects that return a JSON `Error`.
pub type Result<T> = result::Result<T, Error>;
