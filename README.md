# dnsoverhttps - D'oh!

[![crates.io](http://meritbadge.herokuapp.com/dnsoverhttps)](https://crates.io/crates/dnsoverhttps)

Resolve hostnames by sending DNS queries over HTTPS.
It uses `https://1.1.1.1` as the DNS resolver by default, hosted by Cloudflare.
According to Cloudflare it is a privacy-first consumer DNS service.
See <https://1.1.1.1> for more information.

Based on <https://tools.ietf.org/html/draft-ietf-doh-dns-over-https-07>.

## Drawbacks

* When specifing a URL, the hostname has to be specified as well for use in HTTP.
  The TLS Certificate received from the server is validated, but not checked for the correct hostname..
* Only handles A and AAAA records for now (IPv4 & IPv6, this implicitely handles CNAMES when they are resolved recursively)

## Example: Default resolver

```rust
let addr = dnsoverhttps::resolve_host("example.com");
```

## Example: Custom resolver

```rust
let client = dnsoverhttps::Client::from_url_with_hostname("https://172.217.21.110/experimental", "dns.google.com".to_string()).unwrap();
let addr = client.resolve_host("example.com");
```

## CLI Usage

`dnsoverhttps` comes with a small CLI utility providing `host` functionality to resolve hostnames:

```
$ host example.com
example.com has address 2606:2800:220:1:248:1893:25c8:1946
example.com has address 93.184.216.34
```

To install:

```
cargo install dnsoverhttps
```

## License

MIT. See [LICENSE](LICENSE).
