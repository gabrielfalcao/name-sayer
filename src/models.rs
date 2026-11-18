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
    let value = address.to_string().trim_start_matches('.').to_string();
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
                            items.push((name.to_string(), mtch.as_str().to_string()));
                        }
                        None => {}
                    }
                }
                Ok(items)
            }
            None => Err(Error::DomainParseError(self.to_string())),
        }
    }
    pub fn matches_addresss<T: Display>(&self, other: T) -> Result<bool> {
        let other = BlockedAddress::new(other)?;
        Ok(self.matches_other(other)?)
    }
    pub fn matches_other(&self, other: BlockedAddress) -> Result<bool> {
        let self_parts = self.parts()?;
        let other_parts = other.parts()?;
        if self.host()?.matches(other.as_str()) {
            Ok(true)
        } else if self_parts == other_parts {
            Ok(true)
        } else if self.name()?.eq_case(&other.name()?) {
            Ok(true)
        } else if glob_match(self.as_str(), other.as_str()) {
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
