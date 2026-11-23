use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    net::{IpAddr, Ipv4Addr},
};

// use std::str::FromStr;
use fast_glob::glob_match;
use hickory_proto::rr::domain::Name;
// use hickory_proto::rr::domain::Name;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::{Resolver as HickoryResolver, config::ServerGroup};
use hickory_server::net::runtime::TokioRuntimeProvider;
use iocore::{Path, env};
use regex::Regex;

use crate::{
    BlockedAddress,
    UPSTREAM_SERVERS,
    server::DEFAULT_BLOCKING_IP,
    upstream_servers_to_ipv4_addr_vec,
};
// use hickory_proto::rr::LowerName;
// use debug_et_diagnostics::dbg;
use crate::{Error, Result};
#[derive(Debug, Clone)]
pub struct ConfigResolver {
    blocking_ip: Ipv4Addr,
    mapping: BTreeMap<Name, Ipv4Addr>,
    store: BTreeMap<String, Ipv4Addr>,
    blocked_addresses: BTreeSet<BlockedAddress>,
    resolver: HickoryResolver<TokioRuntimeProvider>,
    upstream_servers: Vec<Ipv4Addr>,
}
impl Default for ConfigResolver {
    fn default() -> ConfigResolver {
        ConfigResolver::load(DEFAULT_BLOCKING_IP, UPSTREAM_SERVERS.clone())
            .expect("failed to load default ConfigResolver")
    }
}
pub fn normalize_domain_name<T: Display>(name: T) -> Result<BlockedAddress> {
    Ok(BlockedAddress::new(&name.to_string())?)
}
impl ConfigResolver {
    fn load_default_upstream_servers() -> Result<Vec<(String, Ipv4Addr)>> {
        let mut result = Vec::<(String, Ipv4Addr)>::new();
        let upstream_servers = UPSTREAM_SERVERS.clone();
        for (name, ipv4) in upstream_servers.into_iter() {
            log::trace!("adding upstream server from {name}: {ipv4}");

            result.push((name.to_string(), ipv4));
        }
        Ok(result.clone())
    }
    fn load_upstream_servers(servers: Vec<Ipv4Addr>) -> Result<Vec<(String, Ipv4Addr)>> {
        let mut result = Vec::<Ipv4Addr>::new();
        for ipv4 in servers.clone().into_iter() {
            log::trace!("adding upstream server {ipv4}");
            result.push(ipv4);
        }
        Ok(result.clone())
    }

    pub fn add_upstream_server_ipv4(&mut self, name: String, addr: Ipv4Addr) -> Result<()> {
        self.upstream_servers.push(addr);
        Ok(())
    }
    pub fn register_record(&mut self, name: String, addr: Ipv4Addr) -> Result<()> {
        let name = BlockedAddress::new(&name)?;
        if let Some(x) = self.store.get_mut(name.as_str()) {
            if x.to_string() != addr.to_string() {
                *x = addr;
            }
        } else {
            self.store.insert(name.to_string(), addr);
        }
        Ok(())
    }
    pub fn add_blocked_address<T: Display>(&mut self, name: T) -> Result<()> {
        let name = name.to_string();
        let blocked = BlockedAddress::new(&name)?;
        self.blocked_addresses.insert(blocked);
        Ok(())
    }
    pub fn map_fixed_record(&mut self, name: Name, addr: Ipv4Addr) -> Result<()> {
        self.mapping.insert(name, addr);
        Ok(())
    }
    pub fn new(
        blocking_ip: Ipv4Addr,
        servers: ServerGroup,
        upstream_servers: Vec<(String, Ipv4Addr)>,
        blocked_addresses: BTreeSet<BlockedAddress>,
        resolver: HickoryResolver<TokioRuntimeProvider>,
    ) -> Result<ConfigResolver> {
        let mut resolver = HickoryResolver::builder_with_config(
            ResolverConfig::udp_and_tcp(&servers),
            TokioRuntimeProvider::default(),
        )
        .build()?;

        Ok(ConfigResolver {
            mapping: BTreeMap::new(),
            store: BTreeMap::new(),
            resolver,
            blocked_addresses,
            upstream_servers: upstream_servers_to_ipv4_addr_vec(upstream_servers),
            blocking_ip,
        })
    }
    pub fn load<'a>(
        blocking_ip: Ipv4Addr,
        upstream_servers: Vec<Ipv4Addr>,
    ) -> Result<ConfigResolver> {
        // let upstream_servers = upstream_servers_to_ipv4_addr_vec(upstream_servers);
        let upstream_servers = if upstream_servers.is_empty() {
            ConfigResolver::load_default_upstream_servers()?
        } else {
            ConfigResolver::load_upstream_servers(upstream_servers)?
        };
        let ips = upstream_servers
            .clone()
            .into_iter()
            .map(|(_name, addr)| IpAddr::V4(addr))
            .collect::<Vec<IpAddr>>();
        let servers = ServerGroup { ips: &ips, server_name: "loopback", path: "/__query_path__" };
        let resolver = HickoryResolver::builder_with_config(
            ResolverConfig::udp_and_tcp(&servers),
            TokioRuntimeProvider::default(),
        )
        .build()?;
        let mut store = BTreeMap::new();
        let mut mapping = BTreeMap::new();
        let mut blocked_addresses = BTreeSet::new();
        Ok(ConfigResolver {
            store,
            mapping,
            blocked_addresses,
            resolver,
            upstream_servers: upstream_servers_to_ipv4_addr_vec(upstream_servers),
            blocking_ip,
        })
    }
    pub async fn resolve<I: Display, T: Iterator<Item = I>>(
        &mut self,
        addresses: T,
    ) -> Result<Vec<String>> {
        let mut result = Vec::<String>::new();
        let requested_names = addresses
            .map(|name| match normalize_domain_name(&name) {
                Ok(name) => Some(name),
                Err(error) => {
                    log::error!("failed to parse name {name:#?}: {error}", name = name.to_string());
                    None
                }
            })
            .filter(|name| name.is_some())
            .map(|name| name.unwrap())
            .collect::<Vec<BlockedAddress>>();
        let requests =
            requested_names.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(", ");
        log::warn!("request to resolve: {}", requests);
        for requested_name in requested_names.clone() {
            // let name = Name::from_str(&requested_name)?;
            // let lower_name = LowerName::new(&name);
            if requested_name.to_string().contains("hai8g.com") {
                result.push(DEFAULT_BLOCKING_IP.to_string());
                return Ok(result);
            }
            log::info!(
                "looking for {requested_name:#?} ",
                requested_name = requested_name.to_string()
            );
            for (name, ipv4) in self.store.clone().into_iter() {
                let name = normalize_domain_name(name)?;
                log::debug!(
                    "looking whether {requested_name:#?}
                     is match to {addr:#?}\n",
                    requested_name = requested_name.to_string(),
                    addr = ipv4.to_string(),
                );

                if name.matches_address(&requested_name)? {
                    result.push(DEFAULT_BLOCKING_IP.to_string());
                    log::warn!(
                        "matched name {name} to {requested_name}, blocking access via ip address {ipv4}"
                    );
                    return Ok(result);
                } else {
                    log::warn!(
                        "failed to match name {name} to {requested_name}, proceeding with resolution via upstream"
                    );
                }
            }
            let total_found = result.len();
            if total_found > 0 {
                return Ok(result);
            }
            let mut is_blocked = false;
            for blocked in self.blocked_addresses.clone() {
                if blocked.matches_other(requested_name.clone())? {
                    log::warn!("blocking {requested_name}, to {DEFAULT_BLOCKING_IP}");
                    result.push(DEFAULT_BLOCKING_IP.to_string());
                    is_blocked = true;
                    return Ok(result);
                } else {
                    log::warn!("no blocking found for {requested_name} against {blocked}");
                }
            }

            let upstream = self.resolver.clone();
            // eprintln!("not found in local store, so querying upstream server {upstream:#?}: {requested_name:#?}");
            let lookup_result = upstream.lookup_ip(requested_name.clone()).await?;

            let matched_ips = lookup_result
                .iter()
                .filter(|ipaddr| ipaddr.is_ipv4())
                .map(|ipaddr| match ipaddr {
                    IpAddr::V4(addr) => addr,
                    IpAddr::V6(v6) => {
                        log::error!("IPV6 found for {requested_name} but IPv6 is unacceptable by name-sayer, throwring it away: {v6}");
                        self.blocking_ip

                    },
                })
                .collect::<Vec<Ipv4Addr>>();
            if !is_blocked {
                log::info!(
                    "no match found locally for {requested_name}, upstream resolved to: {ips}",
                    ips = matched_ips
                        .iter()
                        .map(|ip| ip.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                for ip in matched_ips {
                    self.register_record(requested_name.to_string(), ip);
                }
                result.extend(lookup_result.iter().map(|ip| ip.to_string()));
                log::info!("resolved {requested_name} to {result}", result = result.join(", "));
            }
        }
        Ok(result)
    }
}
