#![allow(unused)]
pub(crate) mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[doc(hidden)] pub mod core;
#[doc(inline)] pub use core::setup_logger;

#[doc(hidden)] pub mod config;
#[doc(inline)]
pub use config::{ConfigItem, ConfigResolver, DomainConfig, SinkDomain, SinksConfig};

#[doc(hidden)] pub mod resolvers;
#[doc(inline)] pub use resolvers::{PrintProvider, UpstreamResolver, query};

#[doc(hidden)] pub mod http;
#[doc(inline)] pub use http::{HTTPRequest, HTTPResponse};

#[doc(hidden)] pub mod io;
#[doc(inline)] pub use io::{TokioExecutor, TokioTimer};

#[doc(hidden)] pub mod server;
#[doc(inline)] pub use server::DNSServer;
