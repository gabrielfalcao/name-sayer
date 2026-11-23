#![allow(unused)]
use std::net::Ipv4Addr;
use clap::{ArgAction, Parser};
use color_eyre::eyre::{Report, Result};
use hickory_proto::rr::Name;
use iocore::{Path, env};
use name_sayer::{
    ALL_UPSTREAM_SERVERS,
    BlockedAddress,
    DNSServer,
    UPSTREAM_SERVERS,
    UpstreamServer,
    setup_logger,
    upstream_servers_to_struct,
};
use tabled::{
    Table,
    Tabled,
    assert::assert_table,
    settings::{Alignment, Style, object::Columns},
};
use tokio::net::UdpSocket;
#[derive(Parser, Debug, Clone)]
#[command(author, version)]
pub struct UpstreamServersCli {
    #[arg(
        short,
        long,
    )]
    all: bool,
}

fn main() -> std::result::Result<(), Report> {
    let cli = UpstreamServersCli::parse();
    let ups = if cli.all { ALL_UPSTREAM_SERVERS.clone() } else { UPSTREAM_SERVERS.clone() };
    let servers = upstream_servers_to_struct(ups);
    let mut table = Table::new(servers);
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());
    println!("{table}");
    Ok(())
}
