use std::string::ToString;

use color_eyre::eyre::{Report, Result, WrapErr};
use hickory_proto::rr::LowerName;
use hickory_proto::rr::domain::Name;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct RegexPattern(String);

impl RegexPattern
{
    pub fn new<T: ToString>(pattern: T) -> Result<RegexPattern>
    {
        let pattern = pattern.to_string();
        let _ = Regex::new(&pattern)
            .wrap_err_with(|| format!("invalid regex pattern: {pattern:#?}"))?;
        Ok(RegexPattern(pattern))
    }

    pub fn pattern(&self) -> String
    {
        self.0.to_string()
    }

    pub fn to_regex(&self) -> Result<Regex>
    {
        let pattern = self.pattern();
        Ok(Regex::new(&pattern)
            .wrap_err_with(|| format!("invalid regexp {pattern}"))?)
    }
}
impl std::fmt::Debug for RegexPattern
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let pattern = self.pattern();
        write!(f, "RegexPattern({pattern:#?})")
    }
}
impl std::fmt::Display for RegexPattern
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self.pattern())
    }
}
impl Into<Regex> for RegexPattern
{
    fn into(self) -> Regex
    {
        self.to_regex().unwrap()
    }
}

#[derive(
    Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ResolutionMatcher
{
    DomainLowerName(hickory_proto::rr::LowerName),
    DomainName(hickory_proto::rr::domain::Name),
    DomainString(String),
    Regex(String),
}

#[derive(
    Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct ResolutionSink
{
    pub ip_addr: Ipv4Addr,
    pub matchers: Vec<ResolutionMatcher>,
}
