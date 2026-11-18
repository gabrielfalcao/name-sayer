use std::{fmt::Display, net::AddrParseError};

use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("I/O Error {0}")]
    IOError(#[from] std::io::Error),
    #[error("I/O Error {0}")]
    IOCoreError(#[from] iocore::Error),
    #[error("Runtime Error {0}")]
    RuntimeError(String),
    #[error("Failed to parse address: {0}")]
    Ipv4ParseError(#[from] AddrParseError),
    #[error("invalid value for blocked address: {0}")]
    DomainParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
