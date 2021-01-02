use std::convert::From;

#[derive(Debug)]
pub struct T2Err(pub String);

impl std::fmt::Display for T2Err {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type T2Result<T> = Result<T, T2Err>;

impl From<()> for T2Err {
    fn from(_error: ()) -> Self {
        T2Err("".into())
    }
}

impl From<&str> for T2Err {
    fn from(error: &str) -> Self {
        T2Err(error.to_string())
    }
}

impl From<&String> for T2Err {
    fn from(error: &String) -> Self {
        T2Err(error.to_string())
    }
}

impl From<String> for T2Err {
    fn from(error: String) -> Self {
        T2Err(error)
    }
}

impl From<std::io::Error> for T2Err {
    fn from(error: std::io::Error) -> Self {
        T2Err(error.to_string())
    }
}

impl From<serde_cbor::error::Error> for T2Err {
    fn from(error: serde_cbor::error::Error) -> Self {
        T2Err(error.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for T2Err {
    fn from(error: tokio::sync::mpsc::error::SendError<T>) -> Self {
        T2Err(error.to_string())
    }
}

impl From<std::str::Utf8Error> for T2Err {
    fn from(error: std::str::Utf8Error) -> Self {
        T2Err(error.to_string())
    }
}

impl From<tokio::task::JoinError> for T2Err {
    fn from(error: tokio::task::JoinError) -> Self {
        T2Err(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for T2Err {
    fn from(error: std::num::ParseFloatError) -> Self {
        T2Err(error.to_string())
    }
}