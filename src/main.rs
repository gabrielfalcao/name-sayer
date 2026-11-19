#![allow(unused)]
use std::net::Ipv4Addr;

use clap::{ArgAction, Parser};
use color_eyre::eyre::{Report, Result};
use iocore::{Path, env};
use name_sayer::{BlockedAddress, DNSServer, setup_logger};
use tokio::net::UdpSocket;
#[derive(Parser, Debug, Clone)]
#[command(author, version)]
pub struct Cli {
    #[arg(short, long, default_value = "53")]
    port: u16,

    #[arg(short = 'H', long, default_value = "0.0.0.0")]
    host: Ipv4Addr,

    #[arg(short, long, default_value = "info")]
    log_level_max: log::LevelFilter,

    #[arg(short, long)]
    output_log_file: Option<Path>,

    #[arg(short, long, default_value = "blocklist.txt")]
    blocklist_files: Vec<Path>,

    #[arg(long,
          short = 'B',
          help = "do not terminate resolving blocked names, leaves the dns-client hanging (not recommended)",
          action = ArgAction::SetTrue,
    )]
    block_non_terminate: bool,

    #[arg(short = 't',
          long,
          help = "resolve all blocked names to the ipv4 address designated in --block-terminate-ipv4",
          default_value_t = true,
          action = ArgAction::SetTrue,
    )]
    block_terminate: bool,

    #[arg(
        short = 'i',
        long,
        help = "an IP address to which all blocked names will resolve to",
        default_value = "23.50.77.69"
    )]
    block_terminate_ipv4: Ipv4Addr,
    // #[arg(short = 'H', long)]
    // help: bool,
}

#[derive(Debug, Clone)]
pub struct Application {
    cli: Cli,
    server: DNSServer,
}

impl Default for Application {
    fn default() -> Self {
        Application::new()
    }
}

impl Application {
    pub fn new() -> Application {
        Application { cli: Cli::parse(), server: DNSServer::default() }
    }

    pub async fn serve(&mut self) -> Result<()> {
        let port = self.cli.port;
        let host = self.cli.host;

        let addr = format!("{host}:{port}");
        display_banner(format!("dns server listening on {addr}"));

        let socket = UdpSocket::bind(&addr).await?;
        let server = self.server.clone();

        server.start(socket).await?;
        Ok(())
    }
}

pub fn display_banner<T: std::fmt::Display>(text: T) {
    let beg = format!("\x1b[2J\x1b[3J\x1b[H\x1b[1;38;2;126;237;86m");
    let end = format!("\x1b[1;38;2;255;214;53m");
    eprintln!("{beg}{text}{end}");
}

#[tokio::main]
async fn main() -> std::result::Result<(), Report> {
    let path = Path::from(dirs::config_dir().unwrap()).join("config.yaml");

    let mut app = Application::new();
    app.server.blocking_ip = app.cli.block_terminate_ipv4;
    setup_logger(app.cli.log_level_max)?;
    let mut names = Vec::<String>::new();
    for file in app.cli.blocklist_files.iter() {
        let lines = file
            .read_lines()?
            .into_iter()
            .filter(|l| l.trim().len() > 0)
            .map(|s| BlockedAddress::new(s))
            .filter(|a| a.is_ok())
            .map(|a| a.unwrap())
            .collect::<Vec<BlockedAddress>>();
    }
    for name in names {
        app.server.upstream.register_record(name, app.cli.block_terminate_ipv4)?;
    }
    app.serve().await?;
    Ok(())
}
