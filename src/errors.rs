use std::{fmt::Display, net::AddrParseError};

use hickory_proto::ProtoError;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("I/O Error {0}")]
    IOError(#[from] std::io::Error),
    #[error("I/O Error {0}")]
    IOCoreError(#[from] iocore::Error),
    #[error("I/O Error {0}")]
    HickoryResolverError(#[from] hickory_net::NetError),
    #[error("Runtime Error {0}")]
    RuntimeError(String),
    #[error("Failed to parse address: {0}")]
    Ipv4ParseError(#[from] AddrParseError),
    #[error("Protocol Error: {0}")]
    ProtoError(#[from] ProtoError),
    #[error("Failed to match host pattern: {0}")]
    HostPatternMatchError(#[from] rune_host_match::Error),
    #[error("invalid value for blocked address: {0}")]
    DomainParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Exit {
    Success,
    Error(Error),
}
impl std::process::Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        match &self {
            Exit::Success => std::process::ExitCode::from(0),
            Exit::Error(error) => {
                eprintln!("{}", error);
                std::process::ExitCode::from(1)
            }
        }
    }
}
impl<T> From<std::result::Result<T, Error>> for Exit {
    fn from(result: std::result::Result<T, Error>) -> Exit {
        match result {
            Ok(_) => Exit::Success,
            Err(e) => Exit::Error(e),
        }
    }
}
