//! # dnsoverhttps - D'oh!
//!
//! Resolve hostnames by sending DNS queries over HTTPS.
//! It uses `dns.google.com` to send the DNS query over HTTPS.
//!
//! Based on <https://tools.ietf.org/html/draft-ietf-doh-dns-over-https-03>.
//!
//! ## Drawbacks
//!
//! * TLS Certificate is not checked.
//!   The connection is done using a static IPv4 address for the server.
//!   TLS Certificate validation had to be disabled, as there's currently no way to pass the right
//!   hostname into the request library.
//! * Uses a fixed IP for the `dns.google.com` server. This is not configurable at the moment.
//! * Only handles A and AAAA records for now (IPv4 & IPv6, this implicitely handles CNAMES when they are resolved recursively)
//!
//! ## Example
//!
//! ```
//! let addr = dnsoverhttps::resolve_host("example.com");
//! ```

#![deny(missing_docs)]

extern crate trust_dns;
extern crate trust_dns_proto;
extern crate reqwest;
#[macro_use]
extern crate failure;

use std::str::FromStr;
use std::net::IpAddr;

use reqwest::header::{ContentType, Host};
use reqwest::mime::Mime;

use trust_dns::op::{Message, Query};
use trust_dns::rr::{Name, RecordType};
use trust_dns::rr::RData::*;

mod error;
use error::Error;

const DNS_HOSTNAME : &str = "dns.google.com";
const DNS_QUERY_URL : &str = "https://172.217.21.110/experimental";

/// Resolve the host specified by `host` as a number of `IpAddr`.
///
/// This method queries the server over HTTPS for both IPv4 and IPv6 addresses.
///
/// If the host cannot be found, the vector will be empty.
/// If any errors are encountered during the resolving, the error is returned.
pub fn resolve_host(host: &str) -> Result<Vec<IpAddr>, Error> {
    let mut headers = reqwest::header::Headers::new();
    headers.set(Host::new(DNS_HOSTNAME, None));
    let client = reqwest::Client::builder()
        .danger_disable_hostname_verification()
        .default_headers(headers)
        .build()?;

    let mut ipv6 = resolve_host_family(&client, RecordType::AAAA, host)?;
    let ipv4 = resolve_host_family(&client, RecordType::A, host)?;

    ipv6.extend(ipv4);
    Ok(ipv6)
}

fn resolve_host_family(client: &reqwest::Client, af: RecordType, name: &str) -> Result<Vec<IpAddr>, Error> {
    let qname = Name::from_str(name)?;
    let query = Query::query(qname, af);
    let mut msg = Message::new();
    msg.set_recursion_desired(true);
    msg.add_query(query);

    let qbuf = msg.to_vec()?;

    let wireformat = Mime::from_str("application/dns-udpwireformat").unwrap();

    let mut resp = client.post(DNS_QUERY_URL)
        .header(ContentType(wireformat))
        .body(qbuf)
        .send()?;

    let mut body = Vec::new();
    resp.copy_to(&mut body)?;
    let msg = Message::from_vec(&body)?;

    let results = msg.answers()
        .iter()
        .map(|answer| answer.rdata())
        .flat_map(|data| {
            match *data {
                A(ipv4) => Some(IpAddr::V4(ipv4)),
                AAAA(ipv6) => Some(IpAddr::V6(ipv6)),
                _ => None
            }
        })
        .collect();

    Ok(results)
}
