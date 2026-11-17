use clap::{Parser, Subcommand};
use log::{Level, debug, error, info, log_enabled, trace, warn};

use name_sayer::{Error, Exit, Result, setup_logger};

use name_sayer::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "name-sayer command-line")]
pub struct DnsServerCli {
    #[arg(short, long, default=53)]
    port: u16,
    #[arg(short, long)]
    log_level: log::LevelFilter,
}
impl DnsServerCli {
    pub fn command(&self) -> TopLevelCommand {
        self.command.clone()
    }
}


fn main() -> Exit {
    DnsServerCli::main()
}
