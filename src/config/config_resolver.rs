use std::{
    collections::{BTreeMap, BTreeSet},
    net::{IpAddr, Ipv4Addr},
};

// use std::str::FromStr;
use fast_glob::glob_match;
// use hickory_proto::rr::domain::Name;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::{Resolver as HickoryResolver, config::ServerGroup};
use hickory_server::net::runtime::TokioRuntimeProvider;
use iocore::{Path, env};
use regex::Regex;

// use hickory_proto::rr::LowerName;
// use debug_et_diagnostics::dbg;
use crate::{ConfigItem, DomainConfig};
use crate::{Error, Result};
#[derive(Debug, Clone)]
pub struct ConfigResolver {
    store: BTreeMap<String, Vec<String>>,
    resolver: HickoryResolver<TokioRuntimeProvider>,
}

impl Default for ConfigResolver {
    fn default() -> ConfigResolver {
        ConfigResolver::load_default_opt_conf().expect("ConfigResolver with items")
    }
}

impl ConfigResolver {
    pub fn add_upstream_server_ipv4(&mut self, _addr: Ipv4Addr) -> Result<()> {
        Ok(())
    }
    pub fn add_records<T: std::fmt::Display, I: Iterator<Item = Ipv4Addr>>(
        &mut self,
        name: T,
        records: I,
    ) -> Result<()> {
        let name = name.to_string();
        let records = records.map(|ip| ip.to_string()).collect::<Vec<String>>();
        if let Some(x) = self.store.get_mut(&name) {
            x.extend(records)
        } else {
            self.store.insert(name, records);
        }
        Ok(())
    }
    pub fn load_empty() -> Result<ConfigResolver> {
        let servers = ServerGroup { ips: &[], server_name: "loopback", path: "/__query_path__" };
        let resolver = HickoryResolver::builder_with_config(
            ResolverConfig::udp_and_tcp(&servers),
            TokioRuntimeProvider::default(),
        )
        .build()?;
        let store = BTreeMap::new();
        Ok(ConfigResolver { store, resolver })
    }
    pub fn load_default_opt_conf() -> Result<ConfigResolver> {
        let path =
            Path::new(env::var("CARGO_MANIFEST_DIR")?).join("config.yaml").try_canonicalize();
        Ok(ConfigResolver::load(&path)?)
    }
    pub fn new(
        items: Vec<ConfigItem>,
        upstream_servers: Vec<Ipv4Addr>,
        resolver: HickoryResolver<TokioRuntimeProvider>,
    ) -> ConfigResolver {
        ConfigResolver { store: BTreeMap::new(), resolver }
    }
    pub fn load(path: &Path) -> Result<ConfigResolver> {
        log::info!("ConfigResolver::load({path:#?})", path = path.to_string());
        let upstream_servers = ConfigResolver::load_upstream_servers(path)?;
        let ips = upstream_servers
            .clone()
            .into_iter()
            .map(|addr| IpAddr::V4(addr))
            .collect::<Vec<IpAddr>>();
        // let items = ConfigResolver::load_config_items(path)?;
        let servers = ServerGroup { ips: &ips, server_name: "loopback", path: "/__query_path__" };
        let resolver = HickoryResolver::builder_with_config(
            ResolverConfig::udp_and_tcp(&servers),
            TokioRuntimeProvider::default(),
        )
        .build()?;
        let mut store = BTreeMap::new();
        // for item in items {
        //     store.insert(
        //         item.target.to_string(),
        //         Vec::from_iter(item.domains.iter().map(|domain| domain.to_string())),
        //         // .iter()
        //         // // .map(|ip| ip.parse::<Ipv4Addr>())
        //         // .map(|ip|{
        //         //     let ipaddr = ip.parse::<Ipv4Addr>();
        //         //     let is_ok = ipaddr.is_ok();
        //         //     if !is_ok {
        //         //         eprintln!("invalid ipv4 address: {ip:#?}");
        //         //         eprintln!("invalid object: {ipaddr:#?}");
        //         //     }
        //         //     ipaddr
        //         // })
        //         // .filter(|ipaddr|ipaddr.is_ok())
        //         // .map(|ip| ip.unwrap())
        //         // .collect(),
        //     );
        // }
        Ok(ConfigResolver { store, resolver })
    }
    pub async fn resolve<I: ToString, T: Iterator<Item = I>>(
        &self,
        addresses: T,
    ) -> Result<Vec<String>> {
        let mut result = Vec::<String>::new();
        let addresses = addresses
            .map(|addr| {
                let addr = addr.to_string().trim_end_matches('.').to_string();
                addr.to_string()
            })
            .collect::<Vec<String>>();
        if !addresses.is_empty() {
            eprintln!();
        }
        for value in addresses.iter() {
            let name_value = value.to_string();
            // let name = Name::from_str(&name_value)?;
            // let lower_name = LowerName::new(&name);
            log::info!("looking for {name_value:#?} ", name_value = name_value.to_string());
            // eprintln!("<STORE CONTENTS length={}>",self.store.len());
            for (addr, names) in self.store.iter() {
                log::debug!(
                    "looking whether {name_value:#?}
                     is match to {addr:#?}\n",
                    name_value = name_value.to_string(),
                    addr = addr.to_string(),
                );
                for name in names.iter() {
                    if glob_match(name.to_string(), name_value.to_string()) {
                        log::info!(
                            "is glob_match({name:#?}, {name_value:#?}) ?",
                            name = name.to_string(),
                            name_value = name_value.to_string()
                        );
                        result.push(addr.to_string());
                    } else if name.to_string().contains(&name_value) {
                        log::info!("resolved name {name:#?} to {addr:#?}", addr = addr.to_string());
                    }
                }
            }
            let total_found = result.len();
            if total_found > 0 {
                let result_items =
                    result.iter().map(|ip| format!("- {ip}")).collect::<Vec<String>>().join("\n");
                eprintln!("found {total_found} items:\n{result_items}\n");
            } else {
                log::warn!("no match found locally, querying upstream for {name_value:#?}\n");
                let upstream = self.resolver.clone();
                // eprintln!("not found in local store, so querying upstream server {upstream:#?}: {name_value:#?}");
                let upstream_result = upstream
                    .lookup_ip(value.to_string())
                    .await?
                    .iter()
                    .filter(|ipaddr| ipaddr.is_ipv4())
                    .map(|ipaddr| match ipaddr {
                        IpAddr::V4(addr) => addr.to_string(),
                        IpAddr::V6(_) => unreachable!(),
                    })
                    .collect::<Vec<String>>();
                if upstream_result.is_empty() {
                    log::warn!(
                        "no match for {name_value:#?} found in upstream found in any upstream server"
                    );
                }
                result.extend(upstream_result);
            }
        }
        Ok(result)
    }

    fn load_upstream_servers(_path: &Path) -> Result<Vec<Ipv4Addr>> {
        let mut result = Vec::<Ipv4Addr>::new();
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

            result.push(ipv4);
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
