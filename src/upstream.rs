use std::{net::Ipv4Addr, sync::LazyLock};
pub static UPSTREAM_SERVERS: LazyLock<Vec<(String, Ipv4Addr)>> = LazyLock::new(|| {
    vec![
        ("quad9_security".to_string(), Ipv4Addr::new(9, 9, 9, 9)),
        ("cloudflare".to_string(), Ipv4Addr::new(1, 0, 0, 1)),
        ("cloudflare".to_string(), Ipv4Addr::new(1, 1, 1, 1)),
        ("google".to_string(), Ipv4Addr::new(8, 8, 4, 4)),
        ("google".to_string(), Ipv4Addr::new(8, 8, 8, 8)),
    ]
});

pub static ALL_UPSTREAM_SERVERS: LazyLock<Vec<(String, Ipv4Addr)>> = LazyLock::new(|| {
    vec![
        ("quad9_security".to_string(), Ipv4Addr::new(9, 9, 9, 9)),
        ("cloudflare".to_string(), Ipv4Addr::new(1, 0, 0, 1)),
        ("cloudflare".to_string(), Ipv4Addr::new(1, 1, 1, 1)),
        ("google".to_string(), Ipv4Addr::new(8, 8, 4, 4)),
        ("google".to_string(), Ipv4Addr::new(8, 8, 8, 8)),
        ("adguard".to_string(), Ipv4Addr::new(94, 140, 14, 14)),
        ("adguard".to_string(), Ipv4Addr::new(94, 140, 15, 15)),
        ("comodo".to_string(), Ipv4Addr::new(156, 154, 70, 22)),
        ("comodo".to_string(), Ipv4Addr::new(156, 154, 71, 22)),
        ("comodo_secure".to_string(), Ipv4Addr::new(8, 20, 247, 20)),
        ("comodo_secure".to_string(), Ipv4Addr::new(8, 26, 56, 26)),
        ("dnswatch".to_string(), Ipv4Addr::new(84, 200, 69, 80)),
        ("dnswatch".to_string(), Ipv4Addr::new(84, 200, 70, 40)),
        ("norton_connect_safe".to_string(), Ipv4Addr::new(199, 85, 126, 10)),
        ("norton_connect_safe".to_string(), Ipv4Addr::new(199, 85, 127, 10)),
        ("opendns".to_string(), Ipv4Addr::new(208, 67, 220, 220)),
        ("opendns".to_string(), Ipv4Addr::new(208, 67, 222, 222)),
        ("opendns".to_string(), Ipv4Addr::new(208, 67, 220, 222)),
        ("opendns".to_string(), Ipv4Addr::new(208, 67, 222, 220)),
    ]
});

pub fn upstream_servers_to_ipv4_addr_vec(servers: Vec<(String, Ipv4Addr)>) -> Vec<Ipv4Addr> {
    servers.clone().into_iter().map(|(name, ip)|ip).collect::<Vec<Ipv4Addr>>()
}
