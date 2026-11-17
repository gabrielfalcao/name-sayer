use std::collections::BTreeMap;
use std::net::Ipv4Addr;
use std::str::FromStr;

use iocore::Path;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialOrd, Ord, PartialEq, Eq,
)]
pub struct DomainConfig
{
    pub sinks: BTreeMap<String, Ipv4Addr>,
}
impl Default for DomainConfig
{
    fn default() -> DomainConfig
    {
        DomainConfig {
            sinks: BTreeMap::new(),
        }
    }
}

impl DomainConfig
{
    pub fn iter_domains(&self) -> Result<Vec<(String, Ipv4Addr)>>
    {
        let mut result = Vec::<(String, Ipv4Addr)>::new();
        for (ipv4, domains) in self.sinks.iter()
        {
            result.push((domains.to_string(), ipv4.parse::<Ipv4Addr>()?));
        }
        Ok(result)
    }

    pub fn load<T: Into<Path>>(path: T) -> Result<DomainConfig>
    {
        let path = Into::<Path>::into(path);
        let path_name = path.name();
        let ip_addr = Ipv4Addr::from_str(&path_name).map_err(|error| {
            Error::Ipv4AddrParseError(
                error,
                format!("failed to parse ip v4 address from {path_name}"),
            )
        })?;
        let domains = path
            .read_lines()?
            .into_iter()
            .map(|line| line.trim().to_string())
            .filter(|line| line.len() > 0)
            .collect::<Vec<_>>();
        let mut sinks = BTreeMap::<String, Vec<String>>::new();
        if let Some(current_domains) = sinks.get_mut(&ip_addr.to_string())
        {
            current_domains.extend(domains);
        }
        else
        {
            sinks.insert(ip_addr.to_string(), domains);
        }
        Ok(DomainConfig { sinks })
    }

    pub fn resolve_domain(
        &self,
        given_domain: &str,
    ) -> Option<(String, Ipv4Addr)>
    {
        let given_domain = given_domain.to_string();
        for (domain, ip_addr) in self.iter_domains().unwrap()
        {
            if domain == given_domain
            {
                return Some((domain, ip_addr));
            }
        }
        None
    }
}
