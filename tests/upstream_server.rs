extern crate dnsoverhttps;

use std::net::IpAddr;
use dnsoverhttps::Client;

const DOMAIN : &str = "fnordig.de";

fn valid_ips() -> Vec<IpAddr> {
    vec![
        "2a01:a700:4611::1fac:2a17".parse().unwrap(),
        "31.172.42.23".parse().unwrap(),
    ]
}

#[test]
fn default() {
    let c = Client::default();
    let addrs = c.resolve_host(DOMAIN).expect("hostname should be resolvable.");

    assert_eq!(valid_ips(), addrs);
}

#[test]
fn oneoneoneone() {
    let c = Client::from_url("https://1.1.1.1/.well-known/dns-query")
        .expect("client should have been created from URL");
    let addrs = c.resolve_host(DOMAIN).expect("hostname should be resolvable.");

    assert_eq!(valid_ips(), addrs);
}

#[test]
fn cloudflare() {
    let c = Client::from_url("https://dns.cloudflare.com/.well-known/dns-query")
        .expect("client should have been created from URL");
    let addrs = c.resolve_host(DOMAIN).expect("hostname should be resolvable.");

    assert_eq!(valid_ips(), addrs);
}

#[test]
fn fnordig() {
    let c = Client::from_url("https://fnordig.de/dns-query")
        .expect("client should have been created from URL");
    let addrs = c.resolve_host(DOMAIN).expect("hostname should be resolvable.");

    assert_eq!(valid_ips(), addrs);
}

#[test]
fn cryptosx() {
    let c = Client::from_url("https://doh.crypto.sx/dns-query")
        .expect("client should have been created from URL");
    let addrs = c.resolve_host(DOMAIN).expect("hostname should be resolvable.");

    assert_eq!(valid_ips(), addrs);
}

#[test]
fn google() {
    let c = Client::from_url("https://dns.google.com/experimental")
        .expect("client should have been created from URL");
    let addrs = c.resolve_host(DOMAIN).expect("hostname should be resolvable.");

    assert_eq!(valid_ips(), addrs);
}
