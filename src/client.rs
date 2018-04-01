use std::net::IpAddr;
use std::str::FromStr;

use reqwest::{self, header, Url};
use reqwest::header::ContentType;
use reqwest::mime::Mime;
use trust_dns::op::{Message, Query};
use trust_dns::rr::{Name, RecordType};
use trust_dns::rr::RData::*;

use error::Error;

/// A DoH client
///
/// It will use the configured DoH server to send all resolve queries to.
///
/// ## Example
///
/// ```
/// let client = dnsoverhttps::Client::from_url("https://1.1.1.1/dns-query").unwrap();
/// let addr = client.resolve_host("example.com");
/// ```
pub struct Client {
    url: Url,
    client: reqwest::Client,
}

impl Client {
    /// Create a new DoH client using the given query URL.
    /// The URL's host will be resolved using the system's resolver.
    /// The host will be queried using a `POST` request using the `application/dns-udpwireformat` content-type for the body.
    pub fn from_url(url: &str) -> Result<Client, Error> {
        trace!("New client with url '{}' (needs resolving)", url);
        let url = Url::from_str(url)?;

        let client = reqwest::Client::builder().build()?;

        Ok(Client {
            url: url,
            client: client,
        })
    }

    /// Create a new DoH client using the given query URL and host.
    /// This should be used to bootstrap DoH resolving without the system's resolver. The URL can
    /// contain the host's IP and the hostname is used in the HTTP request.
    /// The host will be queried using a `POST` request using the `application/dns-udpwireformat` content-type for the body.
    ///
    /// ## Caution
    /// This will disable hostname verification of the TLS server certificate.
    /// The certificate is still checked for validity.
    pub fn from_url_with_hostname(url: &str, host: String) -> Result<Client, Error> {
        trace!("New client with url '{}' and host '{}'", url, host);
        let url = Url::from_str(url)?;

        let mut headers = reqwest::header::Headers::new();
        headers.set(header::Host::new(host, None));
        let client = reqwest::Client::builder()
            .danger_disable_hostname_verification()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            url: url,
            client: client,
        })
    }

    /// Resolve the host specified by `host` as a list of `IpAddr`.
    ///
    /// This method queries the configured server over HTTPS for both IPv4 and IPv6 addresses.
    ///
    /// If the host cannot be found, the list will be empty.
    /// If any errors are encountered during the resolving, the error is returned.
    pub fn resolve_host(&self, host: &str) -> Result<Vec<IpAddr>, Error> {
        trace!("Resolving '{}'", host);
        let mut ipv6 = resolve_host_family(&self.client, self.url.clone(), RecordType::AAAA, host)?;
        trace!("Found {} IPv6 addresses", ipv6.len());
        let ipv4 = resolve_host_family(&self.client, self.url.clone(), RecordType::A, host)?;
        trace!("Found {} IPv4 addresses", ipv4.len());

        ipv6.extend(ipv4);
        Ok(ipv6)
    }
}

fn resolve_host_family(client: &reqwest::Client, url: Url, af: RecordType, name: &str) -> Result<Vec<IpAddr>, Error> {
    let qname = Name::from_str(name)?;
    let query = Query::query(qname, af);
    let mut msg = Message::new();
    msg.set_recursion_desired(true);
    msg.add_query(query);

    let qbuf = msg.to_vec()?;

    let wireformat = Mime::from_str("application/dns-udpwireformat").unwrap();

    let mut resp = client.post(url)
        .header(ContentType(wireformat))
        .body(qbuf)
        .send()?;

    let mut body = Vec::new();
    resp.copy_to(&mut body)?;
    trace!("Got response: {:?}", body);
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

impl Default for Client {
    fn default() -> Client {
        const DNS_QUERY_URL : &str = "https://1.1.1.1/dns-query";

        Client::from_url(DNS_QUERY_URL).unwrap()
    }
}
