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

// use hickory_proto::rr::LowerName;
// use debug_et_diagnostics::dbg;
use crate::{Error, Result};
#[derive(Debug, Clone)]
pub struct ConfigResolver {
    store: BTreeMap<Name, Ipv4Addr>,
    resolver: HickoryResolver<TokioRuntimeProvider>,
    upstream_servers: Vec<(String, Ipv4Addr)>,
}
impl Default for ConfigResolver {
    fn default() -> ConfigResolver {
        ConfigResolver::load().unwrap()
    }
}
pub fn normalize_domain_name<T: Display>(name: T) -> Result<Name> {
    Ok(Name::parse(&name.to_string(), None)?)
}
impl ConfigResolver {
    fn load_default_upstream_servers() -> Result<Vec<(String, Ipv4Addr)>> {
        let mut result = Vec::<(String, Ipv4Addr)>::new();
        let upstream_servers = [
            ("quad9_security", Ipv4Addr::new(9, 9, 9, 9)),
            ("cloudflare", Ipv4Addr::new(1, 0, 0, 1)),
            ("cloudflare", Ipv4Addr::new(1, 1, 1, 1)),
            ("google", Ipv4Addr::new(8, 8, 4, 4)),
            ("google", Ipv4Addr::new(8, 8, 8, 8)),
            // // ("adguard", Ipv4Addr::new(94, 140, 14, 14)),
            // // ("adguard", Ipv4Addr::new(94, 140, 15, 15)),
            // // ("comodo", Ipv4Addr::new(156, 154, 70, 22)),
            // // ("comodo", Ipv4Addr::new(156, 154, 71, 22)),
            // // ("comodo_secure", Ipv4Addr::new(8, 20, 247, 20)),
            // // ("comodo_secure", Ipv4Addr::new(8, 26, 56, 26)),
            // // ("dnswatch", Ipv4Addr::new(84, 200, 69, 80)),
            // // ("dnswatch", Ipv4Addr::new(84, 200, 70, 40)),
            // // ("norton_connect_safe", Ipv4Addr::new(199, 85, 126, 10)),
            // // ("norton_connect_safe", Ipv4Addr::new(199, 85, 127, 10)),
            // ("opendns", Ipv4Addr::new(208, 67, 220, 220)),
            // ("opendns", Ipv4Addr::new(208, 67, 222, 222)),
            // ("opendns", Ipv4Addr::new(208, 67, 220, 222)),
            // ("opendns", Ipv4Addr::new(208, 67, 222, 220)),
        ];
        for (name, ipv4) in upstream_servers.into_iter() {
            log::trace!("adding upstream server from {name}: {ipv4}");

            result.push((name.to_string(), ipv4));
        }
        Ok(result.clone())
    }

    pub fn add_upstream_server_ipv4(&mut self, name: String, addr: Ipv4Addr) -> Result<()> {
        self.upstream_servers.push((name, addr));
        Ok(())
    }
    /// Adds a custom ipv4 addr to resolve `name`. This mechanism
    /// allows the flexibility to specify specific domains on top of
    /// the blocked ones and has the potential to replace blocked
    /// addresses
    pub fn register_record(&mut self, name: String, addr: Ipv4Addr) -> Result<()> {
        let name = Name::parse(&name, None)?;
        if let Some(x) = self.store.get_mut(&name) {
            if x.to_string() != addr.to_string() {
                *x = addr;
            }
        } else {
            self.store.insert(name, addr);
        }
        Ok(())
    }
    pub fn new(
        servers: ServerGroup,
        upstream_servers: Vec<(String, Ipv4Addr)>,
        resolver: HickoryResolver<TokioRuntimeProvider>,
    ) -> Result<ConfigResolver> {
        let resolver = HickoryResolver::builder_with_config(
            ResolverConfig::udp_and_tcp(&servers),
            TokioRuntimeProvider::default(),
        )
        .build()?;

        Ok(ConfigResolver { store: BTreeMap::new(), resolver, upstream_servers })
    }
    pub fn load<'a>() -> Result<ConfigResolver> {
        let upstream_servers = ConfigResolver::load_default_upstream_servers()?;
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
        Ok(ConfigResolver { store, resolver, upstream_servers })
    }
    pub async fn resolve<I: Display, T: Iterator<Item = I>>(
        &self,
        addresses: T,
    ) -> Result<Vec<String>> {
        let mut result = Vec::<String>::new();
        let requested_names = addresses
            .map(|name| match normalize_domain_name(&name) {
                Ok(name) => name,
                Err(error) => {
                    log::error!("failed to parse name {name:#?}: {error}",name=name.to_string());
                    Name::new()
                }
            })
            .collect::<Vec<Name>>();
        for requested_name in requested_names.clone() {
            // let name = Name::from_str(&requested_name)?;
            // let lower_name = LowerName::new(&name);
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

                if name.eq_case(&requested_name) {
                    result.push(ipv4.to_string());
                    log::warn!("blocking {requested_name}");
                }
            }
            let total_found = result.len();
            if total_found > 0 {
                return Ok(result);
            }
            log::warn!("no match found locally, querying upstream for {requested_name:#?}\n");
            let upstream = self.resolver.clone();
            // eprintln!("not found in local store, so querying upstream server {upstream:#?}: {requested_name:#?}");
            let lookup_result = upstream.lookup_ip(requested_name.clone()).await?;

            let matched_ips = lookup_result
                .iter()
                .filter(|ipaddr| ipaddr.is_ipv4())
                .map(|ipaddr| match ipaddr {
                    IpAddr::V4(addr) => addr.to_string(),
                    IpAddr::V6(_) => unreachable!(),
                })
                .collect::<Vec<String>>();
            if result.is_empty() {
                log::warn!(
                    "no match for {requested_name:#?} found in upstream found in any upstream server"
                );
            }
            result.extend(result.clone());
            log::info!("resolved {requested_name} to {result}", result = result.join(", "));
        }
        Ok(result)
    }

    // fn load_config_items(path: &Path) -> Result<Vec<ConfigItem>> {
    //     let regexp = Regex::new("^(([0-9]{1,3})[.]([0-9]{1,3})[.]([0-9]{1,3})[.]([0-9]{1,3}))$")?;
    //     let domain_list_file_path = path.clone();
    //     let mut result = Vec::<ConfigItem>::new();
    //     let config = DomainConfig::load(&domain_list_file_path)?;
    //     let name = domain_list_file_path.name();
    //     for (domain, target) in config.iter_domains()? {
    //         dbg!((&domain, &target));
    //     }
    //     // match domain_list_file_path.read()
    //     // {
    //     //     Ok(data) =>
    //     //     {
    //     //         let domains = data
    //     //             .split('\n')
    //     //             .map(|line| line.trim().to_string())
    //     //             .filter(|line| line.len() > 0)
    //     //             .enumerate()
    //     //             .map(|(idx, line)| (idx + 1, line))
    //     //             .collect::<Vec<(usize, String)>>();
    //     //         result.push(ConfigItem {
    //     //             target,
    //     //             domains: BTreeSet::from_iter(
    //     //                 domains.into_iter().map(|(_idx, domain)| domain),
    //     //             ),
    //     //         });
    //     //     },
    //     //     Err(_error) =>
    //     //     {
    //     //         return Err(Error::RuntimeError(format!(
    //     //             "failed to read config file {domain_list_file_path}" // "failed to read config file {domain_list_file_path}: {error}"
    //     //         ))
    //     //         .into());
    //     //     },
    //     // }
    //     Ok(result)
    // }
}
