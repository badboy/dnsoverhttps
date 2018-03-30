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

//#![deny(missing_docs)]

extern crate trust_dns;
extern crate trust_dns_proto;
extern crate reqwest;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

use std::net::IpAddr;

mod error;
mod client;

use error::Error;
pub use client::Client;

/// Resolve the host specified by `host` as a number of `IpAddr`.
///
/// This method queries the server over HTTPS for both IPv4 and IPv6 addresses.
///
/// If the host cannot be found, the vector will be empty.
/// If any errors are encountered during the resolving, the error is returned.
pub fn resolve_host(host: &str) -> Result<Vec<IpAddr>, Error> {
    Client::default().resolve_host(host)
}
