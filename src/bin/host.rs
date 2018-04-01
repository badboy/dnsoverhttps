extern crate dnsoverhttps;
extern crate env_logger;

use dnsoverhttps::Client;

fn main() {
    env_logger::init();

    let mut args = std::env::args().skip(1);
    let name = match args.next() {
        Some(a) => a,
        None => {
            eprintln!("Usage: host hostname [DNS query url]");
            eprintln!();
            eprintln!("The DNS query URL defaults to https://1.1.1.1/dns-query");
            ::std::process::exit(2);
        }
    };

    let client = match args.next() {
        Some(a) => Client::from_url(&a).unwrap(),
        None => Client::default(),
    };

    let addresses = match client.resolve_host(&name) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("An error occured");
            eprintln!("{}", err);
            ::std::process::exit(2);
        }
    };

    if addresses.is_empty() {
        eprintln!("Host {} not found", name);
        ::std::process::exit(1);
    }

    for addr in addresses {
        println!("{} has address {}", name, addr);
    }
}
