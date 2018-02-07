# dnsoverhttps - D'oh!

[![crates.io](http://meritbadge.herokuapp.com/dnsoverhttps)](https://crates.io/crates/dnsoverhttps)

Resolve hostnames by sending DNS queries over HTTPS.
It uses `dns.google.com` to send the base64-encoded DNS query over HTTPS.

Based on <https://tools.ietf.org/html/draft-ietf-doh-dns-over-https-02>.

(A newer version of the draft is available, but the used server supports only version 2 for now)

## Drawbacks

* TLS Certificate is not checked.
  The connection is done using a static IPv4 address for the server.
  TLS Certificate validation had to be disabled, as there's currently no way to pass the right
  hostname into the request library.
* Uses a fixed IP for the `dns.google.com` server. This is not configurable at the moment.
* Only handles A and AAAA records for now (IPv4 & IPv6, this implicitely handles CNAMES when they are resolved recursively)

## Example

```
let addr = dnsoverhttps::resolve_host("example.com");
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
