use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfigItem
{
    pub target: std::net::Ipv4Addr,
    pub domains: BTreeSet<String>,
}
