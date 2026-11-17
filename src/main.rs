#![allow(unused)]
use std::net::Ipv4Addr;

use clap::Parser;
use color_eyre::eyre::{Report, Result};
use iocore::{Path, env};
use name_sayer::{DNSServer, DomainConfig};
use tokio::net::UdpSocket;
#[derive(Parser, Debug, Clone)]
#[command(author, version, disable_help_flag = true)]
pub struct Cli {
    #[arg(short, long, default_value = "53")]
    port: u16,

    #[arg(short, long, default_value = "0.0.0.0")]
    host: Ipv4Addr,

    #[arg(short, long, default_value = "debug")]
    log_level: log::LevelFilter,

    #[arg(short, long)]
    blocklist_files: Vec<Path>,

    #[arg(short,
          long,
          help = "do not terminate resolving blocked names, leaves the dns-client hanging (not recommended)",
          action = ArgAction::SetTrue,
    )]
    block_non_terminate: bool,

    #[arg(short,
          long,
          help = "resolve all blocked names to the ipv4 address designated in --block-terminate-ipv4",
          action = ArgAction::SetTrue,
    )]
    block_terminate: bool,

    #[arg(
        short,
        long,
        help = "an IP address to which all blocked names will resolve to",
        default_value = "23.50.77.69"
    )]
    block_terminate_ipv4: Ipv4Addr,

    #[arg(short = 'H', long)]
    help: bool,
}

#[derive(Debug, Clone)]
pub struct Application {
    cli: Cli,
    server: DNSServer,
    domain_config: DomainConfig,
}

impl Default for Application {
    fn default() -> Self {
        Application::new(DomainConfig::default())
    }
}

impl Application {
    pub fn new(domain_config: DomainConfig) -> Application {
        Application { cli: Cli::parse(), server: DNSServer::default(), domain_config }
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

    let domain_config = DomainConfig::load(path).unwrap_or_default();
    let mut app = Application::new(domain_config);
    app.serve().await?;
    Ok(())
}
