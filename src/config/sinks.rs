use std::collections::BTreeMap;
use std::net::Ipv4Addr;
use std::str::FromStr;

use iocore::Path;
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialOrd, Ord, PartialEq, Eq,
)]
pub struct SinkDomain
{
    pub sink: String,
    pub domains: Vec<String>,
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialOrd, Ord, PartialEq, Eq,
)]
pub struct SinksConfig
{
    pub sinks: BTreeMap<String, Ipv4Addr>,
    pub domains: Vec<SinkDomain>,
}

#[cfg(test)]
mod test
{
    use std::collections::BTreeMap;
    use std::net::Ipv4Addr;

    use k9::assert_equal;

    use super::{SinkDomain, SinksConfig};
    use crate::Result;
    #[test]
    fn test_sinks_config() -> Result<()>
    {
        let input = "
sinks:
  main: 127.0.0.1

domains:
  - sink: main
    domains:
      - '*.edgecastcdn.net'
      - '*.edgecastcdn.net'
      - '*.sentry-cdn.com'
      - '*.phicdn.net'
";

        let mut expected_sinks_map = BTreeMap::new();
        expected_sinks_map.insert("main".to_string(), "127.0.0.1".to_string());
        let mut expected_sink_domain = SinkDomain {
            sink: "main".to_string(),
            domains: vec![
                "*.edgecastcdn.net".to_string(),
                "*.edgecastcdn.net".to_string(),
                "*.sentry-cdn.com".to_string(),
                "*.phicdn.net".to_string(),
            ],
        };
        // let expected_sinks_config = SinksConfig {
        //     sink: "main".to_string(),
        //     domains: vec!
        //     .insert("sink".to_string(), "main".to_string());
        // }

        let result = serde_yaml::from_str::<SinksConfig>(input)?;
        assert_equal!(result.sinks, expected_sinks_map);
        Ok(())
    }
}
