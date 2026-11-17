use std::collections::BTreeMap;
use std::iter::{IntoIterator, Iterator};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::string::ToString;

use color_eyre::eyre::{Report, Result};
use hickory_proto::rr::{LowerName, Name};
use hickory_resolver::Resolver as HickoryResolver;
use hickory_resolver::config::{ServerGroup, *};
use hickory_server::net::runtime::TokioRuntimeProvider;

use crate::Error;
#[derive(Debug, Clone)]
pub struct UpstreamResolver
{
    store: BTreeMap<String, Vec<Ipv4Addr>>,
    upstream_servers: Vec<Ipv4Addr>,
    resolver: HickoryResolver<TokioRuntimeProvider>,
}

impl Default for UpstreamResolver
{
    fn default() -> UpstreamResolver
    {
        UpstreamResolver::new(
            [
                Ipv4Addr::from_octets([1, 1, 1, 1]),
                Ipv4Addr::from_octets([9, 9, 9, 9]),
                Ipv4Addr::from_octets([8, 8, 8, 8]),
                Ipv4Addr::from_octets([8, 8, 4, 4]),
                Ipv4Addr::from_octets([1, 0, 0, 1]),
                // Ipv4Addr::new(127, 0, 0, 1),
            ]
            .into_iter(),
        )
    }
}

impl UpstreamResolver
{
    pub fn add_upstream_server_ipv4(&mut self, addr: Ipv4Addr) -> Result<()>
    {
        if !self.upstream_servers.contains(&addr)
        {
            self.upstream_servers.push(addr);
            Ok(())
        }
        else
        {
            Err(
                Error::RuntimeError(format!("address already added {addr}"))
                    .into(),
            )
        }
    }

    pub fn new<T: Iterator<Item = Ipv4Addr>>(
        upstream_servers: T,
    ) -> UpstreamResolver
    {
        let upstream_servers = upstream_servers.collect::<Vec<Ipv4Addr>>();
        let ips = upstream_servers
            .iter()
            .map(|v4| IpAddr::V4(*v4))
            .collect::<Vec<IpAddr>>();
        let servers = ServerGroup {
            ips: &ips,
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
                .collect::<Vec<Ipv4Addr>>(),
            resolver,
            store: Default::default(),
        }
    }
}

impl UpstreamResolver
{
    pub fn add_records<
        T: ToString,
        // I: Iterator<Item = Ipv4Addr> + bitflags::traits::Flags,
        I: IntoIterator<Item = Ipv4Addr>,
    >(
        &mut self,
        value: T,
        records: I,
    ) -> Result<()>
    {
        let records = records.into_iter().collect::<Vec<Ipv4Addr>>();
        let name_value = value.to_string();
        let name = Name::from_str(&name_value)?;
        let _lower_name = LowerName::from(&name);
        // eprintln!(
        //     "UpstreamResolver.add_records({lower_name}, {})",
        //     records
        //         .iter()
        //         .map(|ip| ip.to_string())
        //         .collect::<Vec<String>>()
        //         .join("\n")
        // );
        if let Some(ip_addrs) = self.store.get_mut(&name_value)
        {
            for addr in records.iter()
            {
                if ip_addrs.contains(addr)
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
    ) -> Result<Vec<Ipv4Addr>, Report>
    {
        let mut result = Vec::<Ipv4Addr>::new();
        let addresses = addresses
            .map(|addr| {
                let addr = addr.to_string().trim_end_matches('.').to_string();
                addr.to_string()
            })
            .collect::<Vec<String>>();
        if !addresses.is_empty()
        {
            eprintln!();
        }
        for value in addresses.iter()
        {
            let name_value = value.to_string();
            eprintln!(
                "\x1b[1;38;2;186;234;72mlooking for \
                 \x1b[1;38;2;186;234;72m\"{name_value}\"\x1b[0m \
                 (src/resolvers/server.rs)"
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
                        .filter(|ipaddr| ipaddr.is_ipv4())
                        .map(|ipaddr| match ipaddr
                        {
                            IpAddr::V4(addr) => addr,
                            IpAddr::V6(_) => unreachable!(),
                        })
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
