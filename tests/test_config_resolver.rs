#![allow(unused)]
use iocore::Path;
use std::net::Ipv4Addr;
use name_sayer::{BLOCKLIST, ConfigResolver, DEFAULT_BLOCKING_IP, Error, Result};
use pretty_assertions::assert_eq;
const BLOCKED_ADDRESSES: [&'static str; 3] =
    ["hai8g.com", "www.qkinm.top", "www.googleadservices.com"];

#[tokio::test]
async fn test_block_address() -> Result<()> {
    let mut resolve = ConfigResolver::load(
        DEFAULT_BLOCKING_IP,
        BLOCKLIST
            .clone()
            .into_iter()
            .map(|a| a.parse::<Ipv4Addr>())
            .filter(|o| o.is_ok())
            .map(|o| o.unwrap())
            .collect::<Vec<Ipv4Addr>>(),
    )?;
    for blocked in BLOCKED_ADDRESSES.iter() {
        resolve.add_blocked_address(blocked)?;
    }
    let result = resolve.resolve(["hai8g.com"].iter()).await?;
    assert_eq!(result, vec!["23.50.77.69"]);

    let result = resolve.resolve(["www.qkinm.top"].iter()).await?;
    assert_eq!(result, vec!["23.50.77.69"]);
    Ok(())
}
