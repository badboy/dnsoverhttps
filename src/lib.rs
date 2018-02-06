extern crate trust_dns;
extern crate trust_dns_proto;
extern crate base64;
extern crate reqwest;
#[macro_use]
extern crate failure;

use std::str::FromStr;
use std::net::IpAddr;

use base64::encode;
use reqwest::header::Host;

use trust_dns::op::{Message, Query};
use trust_dns::rr::{Name, RecordType};
use trust_dns::rr::RData::*;

mod error;
use error::Error;

const DNS_HOSTNAME : &str = "dns.google.com";
const DNS_QUERY_URL : &str = "https://172.217.21.110/experimental";

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
    let encoded = encode(&qbuf);

    let mut resp = client.get(DNS_QUERY_URL)
                           .query(&[
                                  ("ct", ""),
                                  ("body", &encoded)
                           ])
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
