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
use std::sync::LazyLock;

use lazy_mut::LazyMut;
#[doc(inline)]
pub use upstream::{
    ALL_UPSTREAM_SERVERS,
    UPSTREAM_SERVERS,
    UpstreamServer,
    upstream_servers_to_ipv4_addr_vec,
    upstream_servers_to_struct,
};
pub static BLOCKLIST: LazyLock<Vec<String>> = LazyLock::new(|| {
    let blocklist_files = [
        include_str!("blocklists/blocklist._.txt"),
        include_str!("blocklists/blocklist.asterisk.txt"),
        include_str!("blocklists/blocklist.#.txt"),
        include_str!("blocklists/blocklist.0.txt"),
        include_str!("blocklists/blocklist.1.txt"),
        include_str!("blocklists/blocklist.2.txt"),
        include_str!("blocklists/blocklist.3.txt"),
        include_str!("blocklists/blocklist.4.txt"),
        include_str!("blocklists/blocklist.5.txt"),
        include_str!("blocklists/blocklist.6.txt"),
        include_str!("blocklists/blocklist.7.txt"),
        include_str!("blocklists/blocklist.8.txt"),
        include_str!("blocklists/blocklist.9.txt"),
        include_str!("blocklists/blocklist.a.txt"),
        include_str!("blocklists/blocklist.b.txt"),
        include_str!("blocklists/blocklist.c.txt"),
        include_str!("blocklists/blocklist.d.txt"),
        include_str!("blocklists/blocklist.e.txt"),
        include_str!("blocklists/blocklist.f.txt"),
        include_str!("blocklists/blocklist.g.txt"),
        include_str!("blocklists/blocklist.h.txt"),
        include_str!("blocklists/blocklist.i.txt"),
        include_str!("blocklists/blocklist.j.txt"),
        include_str!("blocklists/blocklist.k.txt"),
        include_str!("blocklists/blocklist.l.txt"),
        include_str!("blocklists/blocklist.m.txt"),
        include_str!("blocklists/blocklist.n.txt"),
        include_str!("blocklists/blocklist.o.txt"),
        include_str!("blocklists/blocklist.p.txt"),
        include_str!("blocklists/blocklist.q.txt"),
        include_str!("blocklists/blocklist.r.txt"),
        include_str!("blocklists/blocklist.s.txt"),
        include_str!("blocklists/blocklist.t.txt"),
        include_str!("blocklists/blocklist.u.txt"),
        include_str!("blocklists/blocklist.v.txt"),
        include_str!("blocklists/blocklist.w.txt"),
        include_str!("blocklists/blocklist.x.txt"),
        include_str!("blocklists/blocklist.y.txt"),
        include_str!("blocklists/blocklist.z.txt"),
    ];
    let mut result = Vec::<String>::new();
    for lines in blocklist_files.iter().map(|l| l.split('\n')) {
        for line in lines {
            result.push(line.to_string());
        }
    }
    use fast_glob::glob_match;
    let intercept = BlockedAddress::new("*intercept*.*").unwrap();
    dbg!(glob_match("www.intercept.com", "*intercept*.*"));
    dbg!(glob_match("www.intercept.com.br", "*intercept*.*"));
    result
});
#[doc(hidden)] pub mod regex;
#[doc(inline)] pub use regex::regex;
