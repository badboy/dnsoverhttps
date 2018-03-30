use std::net::IpAddr;
use std::str::FromStr;

use reqwest::{self, header, Url};
use reqwest::header::ContentType;
use reqwest::mime::Mime;
use trust_dns::op::{Message, Query};
use trust_dns::rr::{Name, RecordType};
use trust_dns::rr::RData::*;

use error::Error;

pub struct Client {
    url: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn from_url(url: &str) -> Client {
        trace!("New client with url '{}' (needs resolving)", url);
        let url = Url::from_str(url).expect("Can't parse URL");

        let client = reqwest::Client::builder()
            .build().unwrap();

        Client {
            url: url,
            client: client,
        }
    }

    pub fn from_url_with_hostname(url: &str, host: String) -> Client {
        trace!("New client with url '{}' and host '{}'", url, host);
        let url = Url::from_str(url).expect("Can't parse URL");

        let mut headers = reqwest::header::Headers::new();
        headers.set(header::Host::new(host, None));
        let client = reqwest::Client::builder()
            .danger_disable_hostname_verification()
            .default_headers(headers)
            .build().unwrap();

        Client {
            url: url,
            client: client,
        }
    }

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
        const DNS_HOSTNAME : &str = "dns.google.com";
        const DNS_QUERY_URL : &str = "https://172.217.21.110/experimental";

        Client::from_url_with_hostname(DNS_QUERY_URL, DNS_HOSTNAME.to_string())
    }
}
