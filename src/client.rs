use std::collections::BTreeSet;
use std::future::Future;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::time::Duration;

use hickory_resolver::Resolver;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::lookup_ip::LookupIp;
use hickory_resolver::net::runtime::iocompat::AsyncIoTokioAsStd;
use hickory_resolver::net::runtime::{RuntimeProvider, TokioHandle, TokioTime};
use tokio::net::{TcpSocket, TcpStream, UdpSocket};
use tokio::time::timeout;

use crate::Error;

#[derive(Clone, Default)]
pub struct PrintProvider
{
    handle: TokioHandle,
}

impl RuntimeProvider for PrintProvider
{
    type Handle = TokioHandle;
    type Tcp = AsyncIoTokioAsStd<TcpStream>;
    type Timer = TokioTime;
    type Udp = UdpSocket;

    fn create_handle(&self) -> Self::Handle
    {
        self.handle.clone()
    }

    fn connect_tcp(
        &self,
        server_addr: SocketAddr,
        bind_addr: Option<SocketAddr>,
        wait_for: Option<Duration>,
    ) -> Pin<Box<dyn Send + Future<Output = Result<Self::Tcp, io::Error>>>>
    {
        Box::pin(async move {
            let socket = match server_addr
            {
                SocketAddr::V4(_) => TcpSocket::new_v4(),
                SocketAddr::V6(_) => TcpSocket::new_v6(),
            }?;
            if let Some(bind_addr) = bind_addr
            {
                socket.bind(bind_addr)?;
            }
            socket.set_nodelay(true)?;
            // eprintln!("connect_tcp socket.bind({socket:#?})");
            // eprintln!("connect_tcp socket.bind");
            let future = socket.connect(server_addr);
            let wait_for = wait_for.unwrap_or_else(|| Duration::from_secs(5));
            match timeout(wait_for, future).await
            {
                Ok(Ok(socket)) => Ok(AsyncIoTokioAsStd(socket)),
                Ok(Err(e)) => Err(e),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "TCP connect timed out",
                )),
            }
        })
    }

    fn bind_udp(
        &self,
        local_addr: SocketAddr,
        _server_addr: SocketAddr,
    ) -> Pin<Box<dyn Send + Future<Output = Result<Self::Udp, io::Error>>>>
    {
        // The server_addr parameter is used only when you need to establish a tunnel or something similar.
        // For example, you try to use a http proxy and encapsulate UDP packets inside a TCP stream.
        // eprintln!(
        //     "Create udp local_addr: {local_addr}, server_addr: {server_addr}"
        // );
        Box::pin(async move { UdpSocket::bind(local_addr).await })
    }
}

pub async fn query(
    domain: &str,
    config: ResolverConfig,
) -> Result<LookupIp, Error>
{
    let resolver =
        Resolver::builder_with_config(config, PrintProvider::default())
            .build()?;
    let response = resolver.lookup_ip(domain).await?;
    let ips = BTreeSet::from_iter(response.iter());
    let ip_response = ips
        .into_iter()
        .map(|ip| match ip
        {
            IpAddr::V4(addr) => Some(addr.to_string()),
            IpAddr::V6(_) => None,
        })
        .filter(|ip| ip.is_some())
        .map(|ip| ip.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    // let reset = "\x1b[2J\x1b[3J\x1b[H";
    eprintln!("querying domain \"{domain}\":\n{ip_response}");
    Ok(response)
}
