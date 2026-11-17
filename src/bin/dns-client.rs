use core::net::{IpAddr, Ipv4Addr, SocketAddrV4};

use clap::Parser;
use hickory_resolver::config::{
    ConnectionConfig, NameServerConfig, ProtocolConfig, ResolverConfig,
};
use tokio::runtime::Runtime;
use white_box::dispatch::ParserDispatcher;
use white_box::{Error, Exit, Result, query};

pub fn name_server_config_from_socket_addr_v4(
    addr: SocketAddrV4,
    protocol: ProtocolConfig,
    mut cfg: ConnectionConfig,
) -> NameServerConfig
{
    let ip = *addr.ip();
    let port = addr.port();
    let _protocol = protocol;
    cfg.port = port;
    // // cfg.bind_addr = Some(SocketAddr::V4(addr));
    NameServerConfig::new(IpAddr::V4(ip), true, vec![cfg])
}
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "dns-client command-line"
)]
pub struct Cli
{
    #[arg()]
    domain: String,

    #[arg(
        short = 'H',
        long,
        default_value = "1.1.1.1"
    )]
    host: Ipv4Addr,

    #[arg(
        short = 'P',
        long,
        default_value = "8853"
    )]
    port: u16,
}
impl Cli
{
    pub fn domain(&self) -> String
    {
        self.domain.to_string()
    }
}
impl ParserDispatcher<Error> for Cli
{
    fn dispatch(&self) -> Result<()>
    {
        let domain = self.domain();

        let rt = Runtime::new()?;
        let handle = rt.handle();

        let (response, error_message): (Option<Vec<IpAddr>>, Option<String>) =
            handle.block_on(async move {
                let configs = [
                    name_server_config_from_socket_addr_v4(
                        SocketAddrV4::new(self.host, self.port),
                        ProtocolConfig::Tcp,
                        ConnectionConfig::tcp(),
                    ),
                    name_server_config_from_socket_addr_v4(
                        SocketAddrV4::new(self.host, self.port),
                        ProtocolConfig::Udp,
                        ConnectionConfig::udp(),
                    ),
                ]
                .to_vec();

                let config =
                    ResolverConfig::from_parts(None, Vec::new(), configs);

                query(&domain, config)
                    .await
                    .map(|response| {
                        let addrs = response
                            .iter()
                            // .filter(|ip| ip.is_ipv4())
                            .collect::<Vec<IpAddr>>();
                        (Some(addrs), None)
                    })
                    .unwrap_or_else(|err| {
                        (
                            None,
                            Some(format!(
                                "no result found for {domain:#?}: {err}"
                            )),
                        )
                    })
            });
        if let Some(response) = response
        {
            let ip_addrs_v4 = response
                .iter()
                .map(|ip| ip.to_string())
                .collect::<Vec<String>>();
            println!("{}", ip_addrs_v4.join("\n"));
        }
        else if let Some(response) = error_message
        {
            eprintln!("{response}");
        }
        Ok(())
    }
}

fn main() -> Exit
{
    tracing_subscriber::fmt::init();
    Cli::main()
}
