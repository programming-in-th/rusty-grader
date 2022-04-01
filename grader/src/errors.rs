#[cfg(feature = "backtraces")]
use std::backtrace::Backtrace;
// use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraderError {
    #[error("Cannot decode UTF8 bytes into string: {msg}")]
    InvalidUtf8 {
        msg: String,
        #[cfg(feature = "backtraces")]
        backtrace: Backtrace,
    },
    #[error("Error parsing into type {target_type}: {msg}")]
    ParseErr {
        target_type: String,
        msg: String,
        #[cfg(feature = "backtraces")]
        backtrace: Backtrace,
    },
    #[error("Error piping IO: {msg}")]
    InvalidIo {
        msg: String,
        #[cfg(feature = "backtraces")]
        backtrace: Backtrace,
    }
}
impl GraderError {
    pub fn invalid_utf8(msg: impl ToString) -> Self {
        GraderError::InvalidUtf8 {
            msg: msg.to_string(),
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::capture(),
        }
    }

    pub fn parse_err(target: impl Into<String>, msg: impl ToString) -> Self {
        GraderError::ParseErr {
            target_type: target.into(),
            msg: msg.to_string(),
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::capture(),
        }
    }

    pub fn invalid_io(msg: impl ToString) -> Self {
        GraderError::InvalidIo {
            msg: msg.to_string(),
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<std::string::FromUtf8Error> for GraderError {
    fn from(source: std::string::FromUtf8Error) -> Self {
        Self::invalid_utf8(source)
    }
}

impl From<std::io::Error> for GraderError {
    fn from(source: std::io::Error) -> Self {
        Self::invalid_io(source)
    }
}


impl From<std::num::ParseIntError> for GraderError {
    fn from(source: std::num::ParseIntError) -> Self {
        Self::parse_err("int", source)
    }
}

impl From<std::num::ParseFloatError> for GraderError {
    fn from(source: std::num::ParseFloatError) -> Self {
        Self::parse_err("float", source)
    }
}

pub type GraderResult<T> = core::result::Result<T, GraderError>;