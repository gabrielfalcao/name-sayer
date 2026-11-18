use std::{fmt::Display, net::AddrParseError};

use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
#[derive(Debug, ThisError, Serialize, Deserialize)]
pub enum Error {
    #[error("I/O Error {0}")]
    IOError(String),
    #[error("I/O Error {0}")]
    IOCoreError(String),
    #[error("Runtime Error {0}")]
    RuntimeError(String),
    #[error("invalid value for blocked address: {0}")]
    DomainParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
