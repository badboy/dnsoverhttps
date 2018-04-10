//! # dnsoverhttps - D'oh!
//!
//! Resolve hostnames by sending DNS queries over HTTPS.
//! It uses `https://1.1.1.1` as the DNS resolver by default, hosted by Cloudflare.
//! According to Cloudflare it is a privacy-first consumer DNS service.
//! See <https://1.1.1.1> for more information.
//!
//! Based on <https://tools.ietf.org/html/draft-ietf-doh-dns-over-https-07>.
//!
//! ## Drawbacks
//!
//! * When specifing a URL, the hostname has to be specified as well for use in HTTP.
//!   The TLS Certificate received from the server is validated, but not checked for the correct hostname..
//! * Only handles A and AAAA records for now (IPv4 & IPv6, this implicitely handles CNAMES when they are resolved recursively)
//!
//! ## Example: Default resolver
//!
//! ```
//! let addr = dnsoverhttps::resolve_host("example.com");
//! ```
//!
//! ## Example: Custom resolver
//!
//! ```
//! let client = dnsoverhttps::Client::from_url_with_hostname("https://172.217.21.110/experimental", "dns.google.com".to_string()).unwrap();
//! let addr = client.resolve_host("example.com");
//! ```

#![deny(missing_docs)]

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

/// Resolve the host specified by `host` as a list of `IpAddr`.
///
/// This method queries the pre-defined default server over HTTPS for both IPv4 and IPv6 addresses.
///
/// If the host cannot be found, the list will be empty.
/// If any errors are encountered during the resolving, the error is returned.
///
/// ## Example
///
/// ```
/// let addr = dnsoverhttps::resolve_host("example.com");
/// ```
pub fn resolve_host(host: &str) -> Result<Vec<IpAddr>, Error> {
    Client::default().resolve_host(host)
}
