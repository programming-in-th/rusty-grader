#[cfg(feature = "backtraces")]
use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
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
    },
    #[error("Error indexing into array")]
    InvalidIndex {
        #[cfg(feature = "backtraces")]
        backtrace: Backtrace,
    },
    #[error("Error unwrapping None option")]
    InvalidValue {
        #[cfg(feature = "backtraces")]
        backtrace: Backtrace,
    },
    #[error("Error transforming from PathBuf to String")]
    InvalidToStr {
        #[cfg(feature = "backtraces")]
        backtrace: Backtrace,
    },
    #[error("Error task not found")]
    TaskNotFound {
        #[cfg(feature = "backtraces")]
        backtrace: Backtrace,
    },
    #[error("Unkown: {msg}")]
    Unknown {
        msg: String,
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

    pub fn invalid_index() -> Self {
        GraderError::InvalidIndex {
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::capture(),
        }
    }

    pub fn invalid_value() -> Self {
        GraderError::InvalidValue {
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::capture(),
        }
    }

    pub fn invalid_to_str() -> Self {
        GraderError::InvalidToStr {
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::capture(),
        }
    }
    pub fn task_not_found() -> Self {
        GraderError::TaskNotFound {
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

impl From<anyhow::Error> for GraderError {
    fn from(value: anyhow::Error) -> Self {
        match value.downcast() {
            Ok(x) => x,
            Err(e) => GraderError::Unknown { msg: e.to_string() }
        }
    }
}

pub type GraderResult<T> = core::result::Result<T, GraderError>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_utf8_works_for_strings() {
        let error = GraderError::invalid_utf8("my text");
        match error {
            GraderError::InvalidUtf8 { msg, .. } => {
                assert_eq!(msg, "my text");
            }
            _ => panic!("expect different error"),
        }
    }

    #[test]
    fn invalid_utf8_works_for_errors() {
        let original = String::from_utf8(vec![0x80]).unwrap_err();
        let error = GraderError::invalid_utf8(original);
        match error {
            GraderError::InvalidUtf8 { msg, .. } => {
                assert_eq!(msg, "invalid utf-8 sequence of 1 bytes from index 0");
            }
            _ => panic!("expect different error"),
        }
    }
}
