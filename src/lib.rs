#![allow(unused)]
pub(crate) mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[doc(hidden)] pub mod core;
#[doc(inline)] pub use core::setup_logger;

#[doc(hidden)] pub mod client;
#[doc(inline)] pub use client::{PrintProvider, query};

#[doc(hidden)] pub mod server;
#[doc(inline)] pub use server::DNSServer;

#[doc(hidden)] pub mod resolver;
#[doc(inline)] pub use resolver::ConfigResolver;

#[doc(hidden)] pub mod models;
#[doc(inline)] pub use models::{BlockedAddress, validate_blocked_address};
