use std::{collections::BTreeMap, fmt::Display, str::FromStr, sync::LazyLock};

use fast_glob::glob_match;
use hickory_proto::rr::domain::Name;
use regex::Regex;
use rune_host_match::HostPattern;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct BlockedAddress(String);
static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?<glob>[*]?)(?<root>[.])?(?<names>(?<name>[a-zA-Z0-9_-]+[.])+)(?<last_name>[a-zA-Z0-9_-]+)$",
    )
    .unwrap()
});
pub fn validate_blocked_address<T: Display>(address: T) -> Result<String> {
    let value = address.to_string().trim_matches('.').to_string();
    match Name::parse(&value, None) {
        Ok(value) => Ok(value.to_string()),
        Err(err) => {
            if REGEX.is_match(&value) {
                Ok(value)
            } else {
                Err(Error::DomainParseError(value.to_string()))
            }
        }
    }
}

impl BlockedAddress {
    pub fn new<T: Display>(address: T) -> Result<BlockedAddress> {
        Ok(BlockedAddress(validate_blocked_address(address)?))
    }
    pub fn name(&self) -> Result<Name> {
        Ok(Name::parse(self.as_str(), None)?)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn host(&self) -> Result<HostPattern> {
        Ok(HostPattern::parse(self.as_str())?)
    }
    pub fn parts(&self) -> Result<Vec<(String, String)>> {
        let caps = REGEX.captures(self.as_str());
        match caps {
            Some(caps) => {
                let mut items = Vec::<(String, String)>::new();
                for name in ["glob", "names", "name", "last_name"] {
                    match caps.name(name) {
                        Some(mtch) => {
                            let value = mtch.as_str().to_string();
                            let value = if value.is_empty() { "*".to_string() } else { value };
                            items.push((name.to_string(), value));
                        }
                        None => {}
                    }
                }
                Ok(items)
            }
            None => Err(Error::DomainParseError(self.to_string())),
        }
    }
    pub fn split(&self) -> Vec<String> {
        self.as_str()
            .split(".")
            .map(|p| p.trim_matches('*'))
            .filter(|p| p.trim().len() > 0)
            .map(String::from)
            .collect::<Vec<String>>()
    }
    pub fn join(&self) -> String {
        self.split().join(".")
    }

    pub fn matches_address<T: Display>(&self, other: T) -> Result<bool> {
        let other = BlockedAddress::new(other)?;
        Ok(self.matches_other(other)?)
    }
    pub fn matches_other(&self, other: BlockedAddress) -> Result<bool> {
        let self_parts = self.parts()?;
        let other_parts = other.parts()?;
        let self_join = self.join();
        let other_join = other.join();
        if self.host()?.matches(other.as_str()) {
            Ok(true)
        } else if dbg!(self_parts) == dbg!(other_parts) {
            Ok(true)
        } else if dbg!(self_join) == dbg!(other_join) {
            Ok(true)
        } else if dbg!(self.name()?).eq_case(&dbg!(other.name())?) {
            Ok(true)
        } else if dbg!(self.as_str()).contains(dbg!(other.as_str())) {
            Ok(true)
        } else if dbg!(other.as_str()).contains(dbg!(self.as_str())) {
            Ok(true)
        } else if glob_match(dbg!(self.as_str()), dbg!(other.as_str())) {
            Ok(true)
        } else if glob_match(dbg!(other.as_str()), dbg!(self.as_str())) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Display for BlockedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str(),)
    }
}

impl FromStr for BlockedAddress {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(BlockedAddress::new(s)?)
    }
}

impl From<String> for BlockedAddress {
    fn from(s: String) -> BlockedAddress {
        BlockedAddress(s)
    }
}

impl Into<Name> for BlockedAddress {
    fn into(self) -> Name {
        self.name().expect("proto name")
    }
}
