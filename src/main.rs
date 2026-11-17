use clap::{Parser, Subcommand};

use dns_server::{Error, Exit, Result};

use dns_server::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "name-sayer command-line")]
pub struct DnsServerCli {
    #[command(subcommand)]
    command: TopLevelCommand,
}
impl DnsServerCli {
    pub fn command(&self) -> TopLevelCommand {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for DnsServerCli {
    fn dispatch(&mut self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum TopLevelCommand {
    Hello(HelloOpt),
}
impl SubcommandDispatcher<Error> for TopLevelCommand {
    fn dispatch(&mut self) -> Result<()> {
        match self {
            TopLevelCommand::Hello(op) => op.dispatch()?,
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct HelloOpt {
    #[arg()]
    text: Vec<String>,
}
impl HelloOpt {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
}
impl ArgsDispatcher<Error> for HelloOpt {
    fn dispatch(&mut self) -> Result<()> {
        println!("{}", &self.text());

        Ok(())
    }
}

fn main() -> Exit {
    DnsServerCli::main()
}
