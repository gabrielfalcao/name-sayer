pub mod server;
pub use server::UpstreamResolver;

pub mod client;
pub use client::{PrintProvider, query};
