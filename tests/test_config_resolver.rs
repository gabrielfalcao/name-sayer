#![allow(unused)]
use pretty_assertions::assert_eq;
use iocore::Path;
use name_sayer::{ConfigResolver, DEFAULT_BLOCKING_IP, Error, Result};
const BLOCKED_ADDRESSES: [&'static str; 3] =
    ["hai8g.com", "www.qkinm.top", "www.googleadservices.com"];

#[tokio::test]
async fn test_block_address() -> Result<()> {
    let mut resolve = ConfigResolver::load(DEFAULT_BLOCKING_IP)?;
    for blocked in BLOCKED_ADDRESSES.iter() {
        resolve.add_blocked_address(blocked)?;
    }
    let result = resolve.resolve(["hai8g.com"].iter()).await?;
    assert_eq!(result, vec!["23.50.77.69"]);

    let result = resolve.resolve(["www.qkinm.top"].iter()).await?;
    assert_eq!(result, vec!["23.50.77.69"]);
    Ok(())
}
