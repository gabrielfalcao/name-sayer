# name-sayer

authoritative DNS server with blocklist features


![https://raw.githubusercontent.com/gabrielfalcao/name-sayer/refs/heads/main/docs/logo.png](https://raw.githubusercontent.com/gabrielfalcao/name-sayer/refs/heads/main/docs/logo.png)

## Installing

```bash
cargo install name-sayer
```

## Usage

### Running a simple DNS server

this spins up the simplest DNS server pointing to the default
upstreams mentioned below:

```bash
name-sayer
```


### Running a DNS Server which blocks names

1. Create a text file with one domain or domain pattern per line, say this file is called `blocklist.txt`
2. `name-sayer -b blocklist.txt` this spins up the DNS server which which prevents the DNS resolution to the real server, pointing to a well-klnown DoD ip instead (hack the world!)


## Well-known Upstream nameservers

### Default Upstream Servers

`name-sayer` uses the following default upstream servers:

```
┌────────────────┬─────────┐
│           name │ ip      │
├────────────────┼─────────┤
│ quad9_security │ 9.9.9.9 │
├────────────────┼─────────┤
│     cloudflare │ 1.0.0.1 │
├────────────────┼─────────┤
│     cloudflare │ 1.1.1.1 │
├────────────────┼─────────┤
│         google │ 8.8.4.4 │
├────────────────┼─────────┤
│         google │ 8.8.8.8 │
└────────────────┴─────────┘
```


### All Upstream Servers

These are not used by default but name-sayer can be setup such that those upstream are used

```
┌─────────────────────┬────────────────┐
│                name │ ip             │
├─────────────────────┼────────────────┤
│      quad9_security │ 9.9.9.9        │
├─────────────────────┼────────────────┤
│          cloudflare │ 1.0.0.1        │
├─────────────────────┼────────────────┤
│          cloudflare │ 1.1.1.1        │
├─────────────────────┼────────────────┤
│              google │ 8.8.4.4        │
├─────────────────────┼────────────────┤
│              google │ 8.8.8.8        │
├─────────────────────┼────────────────┤
│             adguard │ 94.140.14.14   │
├─────────────────────┼────────────────┤
│             adguard │ 94.140.15.15   │
├─────────────────────┼────────────────┤
│              comodo │ 156.154.70.22  │
├─────────────────────┼────────────────┤
│              comodo │ 156.154.71.22  │
├─────────────────────┼────────────────┤
│       comodo_secure │ 8.20.247.20    │
├─────────────────────┼────────────────┤
│       comodo_secure │ 8.26.56.26     │
├─────────────────────┼────────────────┤
│            dnswatch │ 84.200.69.80   │
├─────────────────────┼────────────────┤
│            dnswatch │ 84.200.70.40   │
├─────────────────────┼────────────────┤
│ norton_connect_safe │ 199.85.126.10  │
├─────────────────────┼────────────────┤
│ norton_connect_safe │ 199.85.127.10  │
├─────────────────────┼────────────────┤
│             opendns │ 208.67.220.220 │
├─────────────────────┼────────────────┤
│             opendns │ 208.67.222.222 │
├─────────────────────┼────────────────┤
│             opendns │ 208.67.220.222 │
├─────────────────────┼────────────────┤
│             opendns │ 208.67.222.220 │
└─────────────────────┴────────────────┘
```

# DEMO

![https://raw.githubusercontent.com/gabrielfalcao/name-sayer/refs/heads/main/docs/logo-pixel-art.png](https://raw.githubusercontent.com/gabrielfalcao/name-sayer/refs/heads/main/docs/logo-pixel-art.png)



https://github.com/user-attachments/assets/bcb44897-e799-4b02-937a-4aefba2b6a67
