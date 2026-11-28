#![allow(unused)]
use name_sayer::{BlockedAddress, Error, Result};
use pretty_assertions::assert_eq;

#[test]
fn test_blocked_address_parts() -> Result<()> {
    let hotjar = BlockedAddress::new("*.hotjar.com")?;
    assert_eq!(
        hotjar.parts()?,
        vec![
            ("glob".to_string(), "*".to_string()),
            ("names".to_string(), "hotjar.".to_string()),
            ("name".to_string(), "hotjar.".to_string()),
            ("last_name".to_string(), "com".to_string()),
        ]
    );
    Ok(())
}

#[test]
fn test_blocked_address_with_prefix_wildcard() -> Result<()> {
    let hotjar = BlockedAddress::new("*.hotjar.com")?;
    assert_eq!(hotjar.matches_address("www.hotjar.com")?, true);
    assert_eq!(hotjar.matches_address(".hotjar.com")?, true);
    assert_eq!(hotjar.matches_address("hotjar.com")?, true);
    Ok(())
}
#[test]
fn test_blocked_address_without_wildcard() -> Result<()> {
    let hotjar = BlockedAddress::new("hotjar.com")?;
    assert_eq!(hotjar.matches_address("www.hotjar.com")?, true);
    assert_eq!(hotjar.matches_address(".hotjar.com")?, true);
    assert_eq!(hotjar.matches_address("hotjar.com")?, true);
    Ok(())
}

#[test]
fn test_blocked_address_with_tld() -> Result<()> {
    let hotjar = BlockedAddress::new(".gov.br")?;
    assert_eq!(hotjar.matches_address("www.sus.gov.br")?, true);
    Ok(())
}
