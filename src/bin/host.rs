extern crate dnsoverhttps;

fn main() {
    let mut args = std::env::args().skip(1);
    let name = match args.next() {
        Some(a) => a,
        None => {
            eprintln!("Usage: host hostname");
            ::std::process::exit(2);
        }
    };

    let addresses = match dnsoverhttps::resolve_host(&name) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("An error occured");
            eprintln!("{:?}", err);
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
