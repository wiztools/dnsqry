use clap::{Arg, Command};
use std::str::FromStr;
use std::time::Duration;
use trust_dns_resolver::config::*;
use trust_dns_resolver::proto::rr::{RData, RecordType};
use trust_dns_resolver::Resolver;

fn main() {
    let matches = Command::new("dnsqry")
        .version("1.0")
        .about("DNS query tool")
        .arg(
            Arg::new("domain")
                .help("The domain to query")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("record_type")
                .help("The DNS record type (A, AAAA, NS, MX, TXT, CNAME, etc.)")
                .required(true)
                .index(2),
        )
        .get_matches();

    let domain = matches.get_one::<String>("domain").unwrap();
    let record_type_str = matches.get_one::<String>("record_type").unwrap();

    let record_type = match RecordType::from_str(record_type_str) {
        Ok(rt) => rt,
        Err(_) => {
            eprintln!("Error: Invalid record type '{}'", record_type_str);
            std::process::exit(1);
        }
    };

    // Create resolver with default configuration
    let resolver = match Resolver::new(ResolverConfig::default(), ResolverOpts::default()) {
        Ok(resolver) => resolver,
        Err(e) => {
            eprintln!("Error creating resolver: {}", e);
            std::process::exit(1);
        }
    };

    // Perform the DNS lookup
    let response = match resolver.lookup(domain, record_type) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error querying DNS: {}", e);
            std::process::exit(1);
        }
    };

    // Format and print results
    for record in response.records() {
        let ttl = format_ttl(record.ttl());
        let record_type_name = format!("{:?}", record.record_type());
        let name = record.name().to_string();

        match record.data() {
            Some(RData::A(addr)) => {
                println!("{} {} {}   {}", record_type_name, name, ttl, addr);
            }
            Some(RData::AAAA(addr)) => {
                println!("{} {} {}   {}", record_type_name, name, ttl, addr);
            }
            Some(RData::NS(ns)) => {
                println!("{} {} {}   \"{}\"", record_type_name, name, ttl, ns);
            }
            Some(RData::CNAME(cname)) => {
                println!("{} {} {}   \"{}\"", record_type_name, name, ttl, cname);
            }
            Some(RData::MX(mx)) => {
                println!(
                    "{} {} {}   {} \"{}\"",
                    record_type_name,
                    name,
                    ttl,
                    mx.preference(),
                    mx.exchange()
                );
            }
            Some(RData::TXT(txt)) => {
                let txt_data = txt
                    .iter()
                    .map(|bytes| String::from_utf8_lossy(bytes))
                    .collect::<Vec<_>>()
                    .join("");
                println!("{} {} {}   \"{}\"", record_type_name, name, ttl, txt_data);
            }
            Some(RData::SOA(soa)) => {
                println!(
                    "{} {} {}   \"{}\" \"{}\" {} {} {} {} {}",
                    record_type_name,
                    name,
                    ttl,
                    soa.mname(),
                    soa.rname(),
                    soa.serial(),
                    soa.refresh(),
                    soa.retry(),
                    soa.expire(),
                    soa.minimum()
                );
            }
            Some(RData::PTR(ptr)) => {
                println!("{} {} {}   \"{}\"", record_type_name, name, ttl, ptr);
            }
            Some(other) => {
                println!("{} {} {}   {:?}", record_type_name, name, ttl, other);
            }
            None => {
                println!("{} {} {}   (no data)", record_type_name, name, ttl);
            }
        }
    }
}

fn format_ttl(ttl: u32) -> String {
    let duration = Duration::from_secs(ttl as u64);
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}h{:02}m{:02}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m{:02}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
