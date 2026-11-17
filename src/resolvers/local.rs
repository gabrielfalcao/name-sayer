use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::iter::{IntoIterator, Iterator};
use std::net::{IpAddr, Ipv4Addr};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ToString;

use async_trait::async_trait;
use color_eyre::eyre::{Report, Result, WrapErr};
use hickory_proto::ProtoError;
use hickory_proto::op::HeaderCounts;
use hickory_proto::rr::{LowerName, Name};
use hickory_resolver::Resolver as HickoryResolver;
use hickory_resolver::config::{ServerGroup, *};
use hickory_server::net::runtime::TokioRuntimeProvider;
use hickory_server::proto::op::{Header, Metadata, ResponseCode};
use hickory_server::proto::rr::rdata::{A, AAAA};
use hickory_server::proto::rr::{RData, Record};
use hickory_server::server::{
    Request, RequestHandler, ResponseHandler, ResponseInfo,
    Server as ServerFuture,
};
use hickory_server::zone_handler::{MessageRequest, MessageResponseBuilder};
use tokio::net::UdpSocket;

#[derive(Debug, Clone)]
pub struct UpstreamResolver
{
    store: BTreeMap<String, Vec<IpAddr>>,
    upstream_servers: Vec<IpAddr>,
    resolver: HickoryResolver<TokioRuntimeProvider>,
}
impl Default for UpstreamResolver
{
    fn default() -> UpstreamResolver
    {
        UpstreamResolver::new(
            [
                // IpAddr::V4(Ipv4Addr::from_octets([1, 1, 1, 1])),
                // IpAddr::V4(Ipv4Addr::from_octets([9, 9, 9, 9])),
                // IpAddr::V4(Ipv4Addr::from_octets([8, 8, 8, 8])),
                // IpAddr::V4(Ipv4Addr::from_octets([8, 8, 4, 4])),
                // IpAddr::V4(Ipv4Addr::from_octets([1, 0, 0, 1])),
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            ]
            .into_iter(),
        )
    }
}

impl UpstreamResolver
{
    pub fn new<T: Iterator<Item = IpAddr>>(
        upstream_servers: T,
    ) -> UpstreamResolver
    {
        let upstream_servers = upstream_servers.collect::<Vec<IpAddr>>();
        let mut servers = ServerGroup {
            ips: &upstream_servers,
            server_name: "loopback",
            path: "/__query_path__",
        };
        eprintln!(
            "UpstreamResolver.new({})",
            upstream_servers
                .iter()
                .map(|addr| addr.to_string())
                .map(|addr| format!("{addr:#?}"))
                .collect::<Vec<String>>()
                .join(", ")
        );
        // Construct a new UpstreamResolver with default configuration options
        let resolver = HickoryResolver::builder_with_config(
            ResolverConfig::udp_and_tcp(&servers),
            TokioRuntimeProvider::default(),
        )
        .build()
        .unwrap();

        UpstreamResolver {
            upstream_servers: upstream_servers
                .iter()
                .map(Clone::clone)
                .collect::<Vec<IpAddr>>(),
            resolver: resolver,
            store: Default::default(),
        }
    }
}
impl UpstreamResolver
{
    pub fn add_records<T: ToString>(
        &mut self,
        value: T,
        records: Vec<IpAddr>,
    ) -> Result<()>
    {
        let name_value = value.to_string();
        let name = Name::from_str(&name_value)?;
        let lower_name = LowerName::from(&name);
        eprintln!(
            "UpstreamResolver.add_records({lower_name}, {})",
            records
                .iter()
                .map(|ip| ip.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
        if let Some(mut ip_addrs) = self.store.get_mut(&name_value)
        {
            for addr in records.iter()
            {
                if ip_addrs.contains(&addr)
                {
                    eprintln!("name {name:#?} already present in DNS Store");
                }
                else
                {
                    ip_addrs.push(*addr);
                }
            }
        }
        else
        {
            self.store.insert(name_value, records);
        }
        Ok(())
    }

    pub async fn resolve<I: ToString, T: Iterator<Item = I>>(
        &self,
        addresses: T,
    ) -> Result<Vec<IpAddr>, Report>
    {
        let mut result = Vec::<IpAddr>::new();
        let addresses = addresses
            .map(|addr| {
                let addr = addr.to_string().trim_end_matches('.').to_string();
                addr.to_string()
            })
            .collect::<Vec<String>>();
        if addresses.len() > 0
        {
            eprintln!("");
        }
        for value in addresses.iter()
        {
            let name_value = value.to_string();
            eprintln!(
                "\x1b[1;38;2;158;238;247mlooking for \
                 \x1b[1;38;2;113;189;225m\"{name_value}\"\x1b[0m \
                 (src/resolvers/local.rs)"
            );
            let name = Name::from_str(&name_value)?;
            let lower_name = LowerName::from(name);
            eprintln!("{name_value} is {lower_name:#?}");
            result.extend(
                if let Some(addresses) = self.store.get(&name_value)
                {
                    eprintln!("found in store: {name_value:#?}");
                    addresses.clone()
                }
                else
                {
                    eprintln!("NOT FOUND IN STORE: {name_value:#?}");
                    eprintln!("querying upstream server: {name_value:#?}");
                    self.resolver
                        .lookup_ip(value.to_string())
                        .await?
                        .iter()
                        .collect()
                },
            );
        }
        Ok(result)
    }
}

//#[derive(Clone, Debug)]
//pub struct AddressResolution {
//    pub query_address: String,
//    pub ips: Option<Vec<Ipv4Addr>>,
//    pub error: Option<String>,
//}
//impl AddressResolution {
//    pub fn resolved_ipv4_addrs(&self) -> Vec<Ipv4Addr> {
//        self.ips.clone().unwrap_or_default()
//    }
//    pub fn resolved_ip_addrs(&self) -> Vec<IpAddr> {
//        self.resolved_ipv4_addrs()
//            .into_iter()
//            .map(|addr| IpAddr::V4(addr))
//            .collect::<Vec<IpAddr>>()
//    }
//}
