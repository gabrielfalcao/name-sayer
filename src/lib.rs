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
#[doc(inline)] pub use server::{DEFAULT_BLOCKING_IP, DNSServer};

#[doc(hidden)] pub mod resolver;
#[doc(inline)] pub use resolver::ConfigResolver;

#[doc(hidden)] pub mod models;
#[doc(inline)] pub use models::{BlockedAddress, validate_blocked_address};

#[doc(hidden)] pub mod upstream;
#[doc(inline)]
pub use upstream::{ALL_UPSTREAM_SERVERS, UPSTREAM_SERVERS, upstream_servers_to_ipv4_addr_vec};
